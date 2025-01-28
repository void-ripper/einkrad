use std::{
    collections::HashMap,
    ffi::CString,
    sync::{atomic::AtomicU32, Arc, RwLock},
};

use mlua::UserData;

use crate::node::Node;

static ID_POOL: AtomicU32 = AtomicU32::new(1);

#[derive(Clone)]
pub struct DrawableInstances {
    pub matrices: Arc<RwLock<Vec<[f32; 16]>>>,
    pub instances: Arc<RwLock<HashMap<u32, Arc<RwLock<Node>>>>>,
}

pub struct Drawable {
    pub id: u32,
    pub instances: DrawableInstances,
}

impl Drawable {
    pub fn new(filename: &str) -> Self {
        let id = ID_POOL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);

        Self {
            id,
            instances: DrawableInstances {
                matrices: Arc::new(RwLock::new(Vec::new())),
                instances: Arc::new(RwLock::new(HashMap::new())),
            },
        }
    }

    pub fn draw(&self) {
        //let mut matrices = self.instances.matrices.write().unwrap();
        let instaces = self.instances.instances.read().unwrap();
        unsafe {
            gl::DrawElementsInstanced(
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
        unsafe {}
    }
}

pub struct LuaDrawable {
    pub id: u32,
    pub instances: DrawableInstances,
}

impl UserData for LuaDrawable {}
