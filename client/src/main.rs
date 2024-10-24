use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use node::JsNode;
use package::{Message, Package};
use raylib_ffi::{
    colors, enums::ConfigFlags, rl_str, BeginDrawing, ClearBackground, CloseWindow, DrawFPS,
    EndDrawing, InitWindow, SetConfigFlags, SetTargetFPS, WindowShouldClose,
};
use rquickjs::{class::Trace, Class};
use scene::{JsScene, Scene};

mod drawable;
mod light;
mod node;
mod scene;

enum GameMessage {
    SetLevel(u32),
}

#[derive(Trace, Clone)]
#[rquickjs::class]
struct Game {
    #[qjs(skip_trace)]
    tx: Sender<GameMessage>,
    #[qjs(rename = "isServer")]
    is_server: bool,
}

#[rquickjs::methods]
impl Game {
    #[qjs(rename = "setScene")]
    pub fn set_scene(&self, scene: JsScene) {
        self.tx.send(GameMessage::SetLevel(scene.id)).unwrap();
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
            match Package::load(entry.path(), move |c| {
                let globals = c.globals();
                Class::<JsScene>::define(&globals).unwrap();
                Class::<JsNode>::define(&globals).unwrap();

                globals
                    .set(
                        "Game",
                        Game {
                            tx: gtx.clone(),
                            is_server: false,
                        },
                    )
                    .unwrap();
            }) {
                Ok(pk) => {
                    plugins.push(pk);
                }
                Err(e) => {
                    println!("{e}");
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
                        Message::CreateScene(name) => {
                            println!("EINKRAD: create scene {}", name);
                            let s = Scene::new(name);
                            let id = s.id;
                            scenes.insert(s.id, s);
                            pk.service_tx.send(Message::CreatedScene(id)).unwrap();
                        }
                        Message::LoadDrawable(scene_id, file) => {
                            println!("EINKRAD: load drawable {} {}", scene_id, file);
                            if let Some(scene) = scenes.get_mut(&scene_id) {
                                let did = scene.load(file);
                                pk.service_tx.send(Message::LoadedDrawable(did)).unwrap();
                            }
                        }
                        Message::CreatedScene(_) | Message::LoadedDrawable(_) => {
                            println!("we should not get this");
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
            ClearBackground(colors::WHITE);

            if let Some(scene) = scenes.get(&active_scene) {
                scene.draw();
            }

            DrawFPS(20, 20);
            EndDrawing();
        }

        CloseWindow();
    }

    Ok(())
}
