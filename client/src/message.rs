use std::sync::{Arc, RwLock};

use raylib_ffi::Matrix;


#[derive(Clone)]
pub enum ServiceMessage {
    CreateScene(String),
    CreatedScene(u32),
    LoadDrawable(u32, String),
    LoadedDrawable(u32, Arc<RwLock<Vec<Matrix>>>),
}
