use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use message::ServiceMessage;
use mlua::AnyUserData;
use node::{LuaNode, Node};
use package::Package;
use raylib_ffi::{
    enums::ConfigFlags, BeginDrawing, ClearBackground, CloseWindow, Color, DrawFPS, EndDrawing,
    InitWindow, SetConfigFlags, SetTargetFPS, WindowShouldClose,
};
use scene::{lua_scene_new, LuaScene, Scene};

mod drawable;
mod light;
mod message;
mod node;
mod scene;

#[macro_export]
macro_rules! rl_str {
    ($expression:expr) => {
        format!("{}\0", $expression).as_ptr() as *const i8
    };
}

enum GameMessage {
    SetLevel(u32),
}

struct Game {
    tx: Sender<GameMessage>,
    is_server: bool,
}

impl mlua::UserData for Game {
    fn add_fields<F: mlua::UserDataFields<Self>>(fields: &mut F) {
        fields.add_field_method_get("isServer", |_lua, me| Ok(me.is_server));
    }

    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("setScene", |_lua, me, scene: AnyUserData| {
            let id = scene.borrow_scoped(|s: &LuaScene| s.id)?;
            me.tx.send(GameMessage::SetLevel(id)).unwrap();
            Ok(())
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut scenes: HashMap<u32, Scene> = HashMap::new();
    let data: PathBuf = "data".into();
    let mut active_scene = 0;
    let (gtx, grx) = mpsc::channel();

    let mut plugins = Vec::new();

    for entry in data.read_dir()? {
        let entry = entry?;
        if entry.metadata()?.is_dir() {
            let gtx = gtx.clone();
            match Package::<ServiceMessage>::load(entry.path(), move |c| {
                let globals = c.globals();

                globals.set(
                    "Game",
                    Game {
                        tx: gtx.clone(),
                        is_server: false,
                    },
                )?;

                let scene = c.create_table()?;
                let func = c.create_function(lua_scene_new)?;
                scene.set("new", func)?;
                globals.set("Scene", scene)?;

                let node = c.create_table()?;
                let func = c.create_function(|_lua, _: ()| Ok(LuaNode { inner: Node::new() }))?;
                node.set("new", func)?;
                globals.set("Node", node)?;

                Ok(())
            }) {
                Ok(pk) => {
                    plugins.push(pk);
                }
                Err(e) => {
                    println!("EINKRAD: {e}");
                }
            }
        }
    }

    unsafe {
        SetConfigFlags(ConfigFlags::Msaa4xHint as u32);
        InitWindow(1024, 768, rl_str!("Einkrad"));
        SetTargetFPS(60);

        println!("EINKRAD: --- START ---");
        while !WindowShouldClose() {
            for pk in plugins.iter() {
                while let Ok(msg) = pk.service_rx.try_recv() {
                    match msg {
                        ServiceMessage::CreateScene(name) => {
                            println!("EINKRAD: create scene {}", name);
                            let s = Scene::new(name);
                            let id = s.id;
                            let root = s.root.clone();
                            scenes.insert(s.id, s);
                            pk.service_tx
                                .send(ServiceMessage::CreatedScene(id, root))
                                .unwrap();
                        }
                        ServiceMessage::LoadDrawable(scene_id, file) => {
                            println!("EINKRAD: load drawable {} {}", scene_id, file);
                            if let Some(scene) = scenes.get_mut(&scene_id) {
                                let did = scene.load(file);
                                pk.service_tx
                                    .send(ServiceMessage::LoadedDrawable(did.0, did.1))
                                    .unwrap();
                            }
                        }
                        ServiceMessage::CreatedScene(..) | ServiceMessage::LoadedDrawable(..) => {
                            println!("EINKRAD: we should not get this");
                        }
                    }
                }
            }

            while let Ok(msg) = grx.try_recv() {
                match msg {
                    GameMessage::SetLevel(id) => {
                        active_scene = id;
                    }
                }
            }

            BeginDrawing();
            ClearBackground(Color {
                r: 255,
                g: 255,
                b: 255,
                a: 255,
            });

            if let Some(scene) = scenes.get_mut(&active_scene) {
                scene.draw();
            }

            DrawFPS(20, 20);
            EndDrawing();
        }

        CloseWindow();
    }

    Ok(())
}
