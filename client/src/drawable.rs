use std::sync::{atomic::AtomicU32, Arc, RwLock};

use raylib_ffi::{rl_str, DrawMeshInstanced, LoadModel, Material, Matrix, Model, Shader, UnloadModel};
use rquickjs::class::Trace;

static ID_POOL: AtomicU32 = AtomicU32::new(1);

pub struct Drawable {
    pub id: u32,
    model: Model,
    pub matrices: Arc<RwLock<Vec<Matrix>>>,
    material: Material,
}

impl Drawable {
    pub fn new(shader: Shader, filename: &str) -> Self {
        let id = ID_POOL.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        let (model, mat) = unsafe {
            let model = LoadModel(rl_str!(filename));
            let mut mat = *(model.materials.offset(1));
            mat.shader = shader;
            (model, mat)
        };

        Self { id, model, material: mat, matrices: Arc::new(RwLock::new(Vec::new() ))}
    }

    pub fn draw(&self) {
        let matrices = self.matrices.read().unwrap();
        unsafe {
            DrawMeshInstanced(*self.model.meshes.offset(0), self.material, matrices.as_ptr(), matrices.len() as _);
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

#[derive(Trace, Clone)]
#[rquickjs::class]
pub struct JsDrawable {
    #[qjs(skip_trace)]
    pub id: u32,
    #[qjs(skip_trace)]
    pub matrices: Arc<RwLock<Vec<Matrix>>>,
}

#[rquickjs::methods]
impl JsDrawable {}
