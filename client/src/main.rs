use std::{
    collections::HashMap,
    error::Error,
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use message::ServiceMessage;
use node::JsNode;
use package::Package;
use raylib_sys::{
    BeginDrawing, ClearBackground, CloseWindow, Color, ConfigFlags, DrawFPS, EndDrawing,
    InitWindow, SetConfigFlags, SetTargetFPS, WindowShouldClose,
};
use rquickjs::{class::Trace, Class};
use scene::{JsScene, Scene};

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
            match Package::<ServiceMessage>::load(entry.path(), move |c| {
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
        SetConfigFlags(ConfigFlags::FLAG_MSAA_4X_HINT as u32);
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
