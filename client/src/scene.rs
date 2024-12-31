use std::{
    collections::{HashMap, VecDeque},
    ffi::c_int,
    os::raw::c_void,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, RwLock,
    },
};

use mlua::{AnyUserData, UserData};
use package::App;
use raylib_ffi::{
    enums::{CameraProjection, ShaderLocationIndex, ShaderUniformDataType},
    BeginMode3D, Camera, DrawSphereEx, EndMode3D, GetShaderLocation, GetShaderLocationAttrib,
    LoadShader, SetShaderValue, Shader, Vector3,
};

use crate::{
    drawable::{Drawable, DrawableInstances, LuaDrawable},
    light::Light,
    message::ServiceMessage,
    node::{LuaNode, Node},
    rl_str,
};

static ID_POOL: AtomicU32 = AtomicU32::new(1);

pub struct Scene {
    pub id: u32,
    pub name: String,
    pub drawables: HashMap<u32, Drawable>,
    pub root: Arc<RwLock<Node>>,
    lights: Vec<Light>,
    pub camera: Camera,
    shader: Shader,
    view_loc: i32,
}

impl Scene {
    pub fn new(name: String) -> Self {
        let camera = Camera {
            position: Vector3 {
                x: 2.0,
                y: 4.0,
                z: 6.0,
            },
            target: Vector3 {
                x: 0.0,
                y: 0.5,
                z: 0.0,
            },
            up: Vector3 {
                x: 0.0,
                y: 1.0,
                z: 0.0,
            },
            fovy: 45.0,
            projection: CameraProjection::Perspective as i32,
        };

        let shader = unsafe {
            LoadShader(
                // rl_str!("data/lighting_instancing.vs"),
                rl_str!("data/lighting.vs"),
                rl_str!("data/lighting.fs"),
            )
        };

        let view_loc = unsafe {
            let view_loc = shader.locs.offset(ShaderLocationIndex::VectorView as isize);
            *view_loc = GetShaderLocation(shader, rl_str!("viewPos"));
            println!("VIEW: {}", *view_loc);

            let mat_model = shader.locs.offset(ShaderLocationIndex::MatrixMvp as isize);
            *mat_model = GetShaderLocation(shader, rl_str!("mvp"));
            println!("MODEL: {}", *mat_model);

            let mat_model = shader
                .locs
                .offset(ShaderLocationIndex::MatrixModel as isize);
            // *mat_model = GetShaderLocationAttrib(shader, rl_str!("instanceTransform"));
            *mat_model = GetShaderLocation(shader, rl_str!("matModel"));
            println!("INSTANCE: {}", *mat_model);
            // let normal = GetShaderLocationAttrib(shader, rl_str!("vertexNormal"));
            // let normal_loc = shader
            //     .locs
            //     .offset(ShaderLocationIndex::VertexNormal as isize);
            // println!("NORMAL: {} {}", normal, *normal_loc);

            let ambient_loc = GetShaderLocation(shader, rl_str!("ambient"));
            let ambient_value = [0.1f32, 0.1f32, 0.1f32, 1.0f32].as_ptr();
            SetShaderValue(
                shader,
                ambient_loc,
                ambient_value as *const c_void,
                ShaderUniformDataType::Vec4 as i32,
            );

            *view_loc
        };

        let light = Light::new(shader, 0);
        light.update(shader);

        Self {
            id: ID_POOL.fetch_add(1, Ordering::SeqCst),
            name,
            lights: vec![light],
            drawables: HashMap::new(),
            root: Node::new(),
            camera,
            shader,
            view_loc,
        }
    }

    pub fn load(&mut self, file: String) -> (u32, DrawableInstances) {
        let d = Drawable::new(self.shader, &file);
        let id = d.id;
        let matrices = d.instances.clone();
        self.drawables.insert(id, d);
        (id, matrices)
    }

    pub fn draw(&mut self) {
        let mut stack = VecDeque::new();
        {
            let mut r = self.root.write().unwrap();
            r.transform_world = r.transform;
        }
        stack.push_back(self.root.clone());
        while let Some(n) = stack.pop_front() {
            let n = n.read().unwrap();
            for c in n.children.values() {
                let mut ci = c.write().unwrap();
                ci.transform_world = n.transform_world;
                ci.apply_transform();

                if !ci.children.is_empty() {
                    stack.push_back(c.clone());
                }
            }
        }

        unsafe {
            // UpdateCamera(&mut self.camera, enums::CameraMode::Orbital as i32);
            let camera_pos = [
                self.camera.position.x,
                self.camera.position.y,
                self.camera.position.z,
            ]
            .as_ptr();
            SetShaderValue(
                self.shader,
                self.view_loc,
                camera_pos as *mut c_void,
                ShaderUniformDataType::Vec3 as c_int,
            );
            BeginMode3D(self.camera);

            for drw in self.drawables.values() {
                drw.draw();
            }

            DrawSphereEx(self.lights[0].position, 0.2, 8, 8, self.lights[0].color);
            EndMode3D();
        }
    }
}

pub fn lua_scene_new(lua: &mlua::Lua, name: String) -> mlua::Result<LuaScene> {
    let answer = lua
        .named_registry_value::<AnyUserData>("App")?
        .borrow_scoped(|app: &App<ServiceMessage>| {
            app.sync_send(ServiceMessage::CreateScene(name))
        })?;

    if let ServiceMessage::CreatedScene(id, root) = answer {
        Ok(LuaScene {
            id,
            root: LuaNode { inner: root },
        })
    } else {
        Err(mlua::Error::runtime("could not create scene"))
    }
}

pub struct LuaScene {
    pub id: u32,
    pub root: LuaNode,
}

impl UserData for LuaScene {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("root", |_lua, me| Ok(me.root.clone()));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("load", |lua, me, file: String| {
            let answer = lua
                .named_registry_value::<AnyUserData>("App")?
                .borrow_scoped(|app: &App<ServiceMessage>| {
                    app.sync_send(ServiceMessage::LoadDrawable(me.id, file))
                })?;

            if let ServiceMessage::LoadedDrawable(id, instances) = answer {
                Ok(LuaDrawable { id, instances })
            } else {
                Err(mlua::Error::runtime("could not load drawable"))
            }
        });
    }
}
