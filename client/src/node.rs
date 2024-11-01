use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, RwLock,
    },
};

use raylib_sys::Matrix;
use rquickjs::class::Trace;

use crate::drawable::{DrawableInstances, JsDrawable};

static ID_POOL: AtomicU32 = AtomicU32::new(1);

pub struct Node {
    id: u32,
    parent: Option<Arc<RwLock<Node>>>,
    pub(crate) children: HashMap<u32, Arc<RwLock<Node>>>,
    pub(crate) transform: [f32; 16],
    pub(crate) transform_world: [f32; 16],
    drawable: Option<DrawableInstances>,
}

impl Node {
    pub fn new() -> Arc<RwLock<Self>> {
        let mut transform = [0.0; 16];
        common::matrix::identity(&mut transform);

        Arc::new(RwLock::new(Self {
            id: ID_POOL.fetch_add(1, Ordering::SeqCst),
            parent: None,
            children: HashMap::new(),
            transform,
            transform_world: [0.0; 16],
            drawable: None,
        }))
    }

    pub fn apply_transform(&mut self) {
        common::matrix::mul_assign(&mut self.transform_world, &self.transform);
    }
}

#[derive(Trace, Clone)]
#[rquickjs::class(rename = "Node")]
pub struct JsNode {
    #[qjs(skip_trace)]
    pub inner: Arc<RwLock<Node>>,
}

#[rquickjs::methods]
impl JsNode {
    #[qjs(constructor)]
    pub fn new() -> Self {
        let n = Node::new();
        Self { inner: n }
    }

    #[qjs(rename = "setDrawable")]
    pub fn set_drawable(&self, drw: JsDrawable) {
        let instances = drw.instances.clone();
        let id = self.inner.read().unwrap().id;
        instances
            .instances
            .write()
            .unwrap()
            .insert(id, self.inner.clone());
        let m = Matrix {
            m0: 1.0,
            m1: 0.0,
            m2: 0.0,
            m3: 0.0,
            m4: 0.0,
            m5: 1.0,
            m6: 0.0,
            m7: 0.0,
            m8: 0.0,
            m9: 0.0,
            m10: 1.0,
            m11: 0.0,
            m12: 0.0,
            m13: 0.0,
            m14: 0.0,
            m15: 1.0,
        };
        instances.matrices.write().unwrap().push(m);
        self.inner.write().unwrap().drawable = Some(instances);
    }

    pub fn add(&self, child: JsNode) {
        let mut c = child.inner.write().unwrap();

        if let Some(op) = &c.parent {
            op.write().unwrap().children.remove(&c.id);
        }

        c.parent = Some(self.inner.clone());

        self.inner
            .write()
            .unwrap()
            .children
            .insert(c.id, child.inner.clone());
    }

    pub fn translate(&self, m: Vec<f32>) {
        let mut n = self.inner.write().unwrap();
        let x = [m[0], m[1], m[2]];
        common::matrix::translate(&mut n.transform, &x);
    }

    #[qjs(rename = "rotateX")]
    pub fn rotate_x(&self, a: f32) {
        let mut n = self.inner.write().unwrap();
        common::matrix::rotate_x(&mut n.transform, a);
    }

    #[qjs(rename = "rotateY")]
    pub fn rotate_y(&self, a: f32) {
        let mut n = self.inner.write().unwrap();
        common::matrix::rotate_y(&mut n.transform, a);
    }

    #[qjs(rename = "rotateZ")]
    pub fn rotate_z(&self, a: f32) {
        let mut n = self.inner.write().unwrap();
        common::matrix::rotate_z(&mut n.transform, a);
    }
}
