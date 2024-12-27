use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU32, Ordering},
        Arc, RwLock,
    },
};

use mlua::AnyUserData;
use raylib_ffi::Matrix;

use crate::drawable::{DrawableInstances, LuaDrawable};

static ID_POOL: AtomicU32 = AtomicU32::new(1);

pub struct Node {
    id: u32,
    parent: Option<Arc<RwLock<Node>>>,
    pub children: HashMap<u32, Arc<RwLock<Node>>>,
    pub transform: [f32; 16],
    pub transform_world: [f32; 16],
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

#[derive(Clone)]
pub struct LuaNode {
    pub inner: Arc<RwLock<Node>>,
}

impl mlua::UserData for LuaNode {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("setDrawable", |_lua, me, drw: AnyUserData| {
            let instances = drw.borrow_scoped(|drw: &LuaDrawable| drw.instances.clone())?;
            let id = me.inner.read().unwrap().id;
            instances
                .instances
                .write()
                .unwrap()
                .insert(id, me.inner.clone());
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
            me.inner.write().unwrap().drawable = Some(instances);
            Ok(())
        });

        methods.add_method("add", |_lua, me, child: AnyUserData| {
            let child = child.borrow_scoped(|child: &LuaNode| child.inner.clone())?;
            let mut c = child.write().unwrap();

            if let Some(op) = &c.parent {
                op.write().unwrap().children.remove(&c.id);
            }

            c.parent = Some(me.inner.clone());

            me.inner
                .write()
                .unwrap()
                .children
                .insert(c.id, child.clone());
            Ok(())
        });

        methods.add_method("translate", |_lua, me, m: Vec<f32>| {
            let mut n = me.inner.write().unwrap();
            let x = [m[0], m[1], m[2]];
            common::matrix::translate(&mut n.transform, &x);
            Ok(())
        });

        methods.add_method("rotateX", |_lua, me, a: f32| {
            let mut n = me.inner.write().unwrap();
            common::matrix::rotate_x(&mut n.transform, a);
            Ok(())
        });

        methods.add_method("rotateY", |_lua, me, a: f32| {
            let mut n = me.inner.write().unwrap();
            common::matrix::rotate_y(&mut n.transform, a);
            Ok(())
        });

        methods.add_method("rotateZ", |_lua, me, a: f32| {
            let mut n = me.inner.write().unwrap();
            common::matrix::rotate_z(&mut n.transform, a);
            Ok(())
        });

        methods.add_method("scale", |_lua, me, a: Vec<f32>| {
            let mut n = me.inner.write().unwrap();
            let v = [a[0], a[1], a[2]];
            common::matrix::scale(&mut n.transform, &v);
            Ok(())
        });
    }
}
