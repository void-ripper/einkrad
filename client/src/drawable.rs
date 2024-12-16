use std::{
    collections::HashMap,
    sync::{atomic::AtomicU32, Arc, RwLock},
};

use mlua::{FromLua, UserData};
use raylib_sys::{DrawMeshInstanced, LoadModel, Material, Matrix, Model, Shader, UnloadModel};

use crate::{node::Node, rl_str};

static ID_POOL: AtomicU32 = AtomicU32::new(1);

fn matrix_2_raylib(m: &[f32; 16], r: &mut Matrix) {
    r.m0 = m[0];
    r.m1 = m[1];
    r.m2 = m[2];
    r.m3 = m[3];
    r.m4 = m[4];
    r.m5 = m[5];
    r.m6 = m[6];
    r.m7 = m[7];
    r.m8 = m[8];
    r.m9 = m[9];
    r.m10 = m[10];
    r.m11 = m[11];
    r.m12 = m[12];
    r.m13 = m[13];
    r.m14 = m[14];
    r.m15 = m[15];
}

#[derive(Clone)]
pub struct DrawableInstances {
    pub matrices: Arc<RwLock<Vec<Matrix>>>,
    pub instances: Arc<RwLock<HashMap<u32, Arc<RwLock<Node>>>>>,
}

pub struct Drawable {
    pub id: u32,
    model: Model,
    material: Material,
    pub instances: DrawableInstances,
}

impl Drawable {
    pub fn new(shader: Shader, filename: &str) -> Self {
        let id = ID_POOL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let (model, mat) = unsafe {
            let model = LoadModel(rl_str!(filename));
            let mut mat = *(model.materials.offset(0));
            mat.shader = shader;
            (model, mat)
        };

        Self {
            id,
            model,
            material: mat,
            instances: DrawableInstances {
                matrices: Arc::new(RwLock::new(Vec::new())),
                instances: Arc::new(RwLock::new(HashMap::new())),
            },
        }
    }

    pub fn draw(&self) {
        let mut matrices = self.instances.matrices.write().unwrap();
        let instaces = self.instances.instances.read().unwrap();

        for (i, n) in instaces.values().enumerate() {
            let n = n.read().unwrap();
            matrix_2_raylib(&n.transform_world, &mut matrices[i]);
        }
        unsafe {
            DrawMeshInstanced(
                *self.model.meshes.offset(0),
                self.material,
                matrices.as_ptr(),
                matrices.len() as _,
            );
        }
    }
}

impl Drop for Drawable {
    fn drop(&mut self) {
        unsafe {
            UnloadModel(self.model);
        }
    }
}

pub struct LuaDrawable {
    pub id: u32,
    pub instances: DrawableInstances,
}

impl UserData for LuaDrawable {}
