use std::{
    collections::HashMap,
    os::raw::c_void,
    sync::atomic::{AtomicU32, Ordering},
};

use package::{App, Message};
use raylib_ffi::{
    colors, enums, rl_str, BeginMode3D, Camera, DrawModel, EndMode3D, GetShaderLocation,
    SetShaderValue, Shader, Vector3,
};
use rquickjs::{class::Trace, Ctx, Exception};

use crate::{
    drawable::{Drawable, JsDrawable},
    light::Light,
};

static ID_POOL: AtomicU32 = AtomicU32::new(1);

pub struct Scene {
    pub id: u32,
    pub name: String,
    pub drawables: HashMap<u32, Drawable>,
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
            projection: enums::CameraProjection::Perspective as i32,
        };

        let shader = unsafe {
            raylib_ffi::LoadShader(
                raylib_ffi::rl_str!("data/lighting.vs"),
                raylib_ffi::rl_str!("data/lighting.fs"),
            )
        };

        let view_loc = unsafe {
            let view_loc = shader
                .locs
                .offset(enums::ShaderLocationIndex::VectorView as isize);
            *view_loc = GetShaderLocation(shader, rl_str!("viewPos"));

            let mat_model = shader
                .locs
                .offset(enums::ShaderLocationIndex::MatrixModel as isize);
            *mat_model = GetShaderLocation(shader, rl_str!("matModel"));

            let ambient_loc = GetShaderLocation(shader, rl_str!("ambient"));
            let ambient_value = [0.1f32, 0.1f32, 0.1f32, 1.0f32].as_ptr();
            SetShaderValue(
                shader,
                ambient_loc,
                ambient_value as *const c_void,
                enums::ShaderUniformDataType::Ivec4 as i32,
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
            camera,
            shader,
            view_loc,
        }
    }

    pub fn load(&mut self, file: String) -> u32 {
        let d = Drawable::new(self.shader.clone(), &file);
        let id = d.id;
        self.drawables.insert(id, d);
        id
    }

    pub fn draw(&self) {
        unsafe {
            BeginMode3D(self.camera);

            for drw in self.drawables.values() {
                DrawModel(
                    drw.model,
                    Vector3 {
                        x: 0.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    1.0,
                    colors::WHITE,
                );
            }

            EndMode3D();
        }
    }
}

#[derive(Trace, Clone)]
#[rquickjs::class(rename = "Scene")]
pub struct JsScene {
    #[qjs(skip_trace)]
    pub id: u32,
}

#[rquickjs::methods]
impl JsScene {
    #[qjs(constructor)]
    pub fn new<'js>(ctx: Ctx<'js>, name: String) -> rquickjs::Result<Self> {
        let app: App = ctx.globals().get("App").unwrap();
        let answer = app.sync_send(Message::CreateScene(name));

        if let Message::CreatedScene(id) = answer {
            Ok(Self { id })
        } else {
            Err(Exception::throw_message(&ctx, "could not create scene"))
        }
    }

    pub fn load<'js>(&self, ctx: Ctx<'js>, file: String) -> rquickjs::Result<JsDrawable> {
        let app: App = ctx.globals().get("App").unwrap();
        let answer = app.sync_send(Message::LoadDrawable(self.id, file));

        if let Message::LoadedDrawable(id) = answer {
            Ok(JsDrawable { id })
        } else {
            Err(Exception::throw_message(&ctx, "could not load drawable"))
        }
    }
}
