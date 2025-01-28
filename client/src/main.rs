use std::{
    collections::HashMap,
    error::Error,
    ffi::CString,
    path::PathBuf,
    sync::mpsc::{self, Sender},
};

use message::ServiceMessage;
use mlua::AnyUserData;
use node::{LuaNode, Node};
use package::Package;
use scene::{lua_scene_new, LuaScene, Scene};
use sdl2::{event::Event, keyboard::Keycode};

mod drawable;
mod light;
mod message;
mod node;
mod scene;

enum GameMessage {
    SetLevel(u32),
    SetTargetFPS(u32),
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

        methods.add_method("setTargetFPS", |_lua, me, fps: u32| {
            me.tx.send(GameMessage::SetTargetFPS(fps)).unwrap();
            Ok(())
        });
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let sdl_context = sdl2::init()?;

    let mut scenes: HashMap<u32, Scene> = HashMap::new();
    let data: PathBuf = "data".into();
    let mut active_scene = 0;
    let (gtx, grx) = mpsc::channel();

    let mut plugins = Vec::new();

    // Hints: https://wiki.libsdl.org/SDL2/CategoryHints
    sdl2::hint::set("SDL_GL_MULTISAMPLEBUFFERS", "1");
    sdl2::hint::set("SDL_GL_MULTISAMPLESAMPLES", "4");

    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    let window = video_subsystem
        .window("Einkrad", 1024, 768)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

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

    println!("EINKRAD: --- START ---");
    let mut event_pump = sdl_context.event_pump()?;
    'main: loop {
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
                GameMessage::SetTargetFPS(fps) => {
                    // SetTargetFPS(fps as _);
                }
            }
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                } => break 'main,
            }
        }
        if let Some(scene) = scenes.get_mut(&active_scene) {
            scene.update();
        }

        gl::ClearColor(0.5, 0.5, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        if let Some(scene) = scenes.get_mut(&active_scene) {
            scene.draw();
        }

        window.gl_swap_window();
    }

    Ok(())
}
