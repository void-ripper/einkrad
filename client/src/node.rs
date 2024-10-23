use std::sync::{atomic::{AtomicU64, Ordering}, Arc, RwLock};

use rquickjs::class::Trace;

static ID_POOL: AtomicU64 = AtomicU64::new(1);

pub struct Node {
    id: u64,
}

impl Node {
    pub fn new() -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            id: ID_POOL.fetch_add(1, Ordering::SeqCst),
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
        Self {
            inner: Node::new(),
        }
    }
}
