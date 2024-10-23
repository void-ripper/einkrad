use std::sync::atomic::AtomicU32;

use raylib_ffi::{rl_str, LoadModel, Model, Shader, UnloadModel};
use rquickjs::class::Trace;

static ID_POOL: AtomicU32 = AtomicU32::new(1);

pub struct Drawable {
    pub id: u32,
    pub model: Model,
}

impl Drawable {
    pub fn new(shader: Shader, filename: &str) -> Self {
        let id = ID_POOL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let model = unsafe {
            let model = LoadModel(rl_str!(filename));
            (*(model.materials.offset(1))).shader = shader;
            model
        };

        Self { id, model }
    }
}

impl Drop for Drawable {
    fn drop(&mut self) {
        unsafe {
            UnloadModel(self.model);
        }
    }
}

#[derive(Trace, Clone)]
#[rquickjs::class]
pub struct JsDrawable {
    #[qjs(skip_trace)]
    pub id: u32,
}

#[rquickjs::methods]
impl JsDrawable {}
