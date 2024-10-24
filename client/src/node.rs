use std::{collections::HashMap, sync::{atomic::{AtomicU32, Ordering}, Arc, RwLock}};

use rquickjs::class::Trace;

use crate::drawable::JsDrawable;

static ID_POOL: AtomicU32 = AtomicU32::new(1);

pub struct Node {
    id: u32,
    parent: Option<Arc<RwLock<Node>>>,
    children: HashMap<u32, Arc<RwLock<Node>>>,
    drawable: Option<u32>,
}

impl Node {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            id: ID_POOL.fetch_add(1, Ordering::SeqCst),
            parent: None,
            children: HashMap::new(),
            drawable: None,
        }))
    }
}

#[derive(Trace, Clone)]
#[rquickjs::class(rename = "Node")]
pub struct JsNode {
    #[qjs(skip_trace)]
    inner: Arc<RwLock<Node>>,
}

#[rquickjs::methods]
impl JsNode {
    #[qjs(constructor)]
    pub fn new() -> Self {
        let n = Node::new();
        Self {
            inner: n,
        }
    }

    #[qjs(rename = "setDrawable")]
    pub fn set_drawable(&self, drw: JsDrawable) {
        self.inner.write().unwrap().drawable = Some(drw.id);
        
    }
}
