use std::sync::{Arc, RwLock};

use crate::{drawable::DrawableInstances, node::Node};

#[derive(Clone)]
pub enum ServiceMessage {
    CreateScene(String),
    CreatedScene(u32, Arc<RwLock<Node>>),
    LoadDrawable(u32, String),
    LoadedDrawable(u32, DrawableInstances),
}
