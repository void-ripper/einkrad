use std::{
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    time::{Duration, SystemTime},
};

use error::PackageError;
use mlua::{Error, FromLua, Lua, UserData};

mod error;

pub struct App<M> {
    service_tx: Sender<M>,
    service_rx: Arc<Receiver<M>>,
}

impl<M> App<M> {
    pub fn sync_send(&self, msg: M) -> M {
        self.service_tx.send(msg).unwrap();
        self.service_rx.recv().unwrap()
    }
}

impl<M> UserData for App<M> {}

pub struct Package<M> {
    pub msg_tx: Sender<String>,
    pub service_rx: Receiver<M>,
    pub service_tx: Sender<M>,
}

fn run_package<F, M: 'static>(
    cb: F,
    root: PathBuf,
    msg_rx: Receiver<String>,
    service_tx: Sender<M>,
    service_rx: Receiver<M>,
) -> Result<(), PackageError>
where
    F: Fn(&Lua),
{
    let indexlua = root.join("index.luau");
    let rt = Lua::new();

    {
        let globals = rt.globals();
        let print = rt.create_function(|_, args: mlua::Value| {
            println!("{:?}", args);
            Ok(())
        })?;
        globals.set("print", print)?;
        let require = rt.create_function(move |_, filename: String| {
            println!("load {}", root.join(filename).display());
            Ok(())
        })?;
        globals.set("require", require)?;

        cb(&rt);
    }

    rt.set_named_registry_value(
        "App",
        App {
            service_tx,
            service_rx: Arc::new(service_rx),
        },
    )?;

    let data = std::fs::read_to_string(&indexlua)?;
    rt.load(&data).exec()?;

    let on_start: mlua::Function = rt.globals().get("OnStart")?;

    let _: () = on_start.call(())?;

    let on_message: mlua::Function = rt.globals().get("OnMessage")?;
    let on_update: mlua::Function = rt.globals().get("OnUpdate")?;

    loop {
        let start = SystemTime::now();

        while let Ok(msg) = msg_rx.try_recv() {
            let _: () = on_message.call((msg,))?;
        }

        let _: () = on_update.call(())?;

        let elapsed = start.elapsed().unwrap();
        let to_wait = Duration::from_millis(100) - elapsed;
        std::thread::sleep(to_wait);
    }
}

impl<M> Package<M>
where
    M: Send + 'static,
{
    pub fn load<F>(root: PathBuf, cb: F) -> Result<Package<M>, PackageError>
    where
        F: Fn(&Lua) + Send + 'static,
    {
        println!("PACKAGE: load {}", root.display());
        let indexlua = root.join("index.luau");

        if !indexlua.exists() {
            return Err(PackageError::not_a_package());
        }

        let (tx, rx) = mpsc::channel();
        let (rtx, rrx) = mpsc::channel();
        let (atx, arx) = mpsc::channel();

        std::thread::spawn(move || {
            if let Err(e) = run_package(cb, root.clone(), rx, rtx, arx) {
                println!("PACKAGE {}: {}", root.display(), e);
            }
        });

        Ok(Self {
            msg_tx: tx,
            service_tx: atx,
            service_rx: rrx,
        })
    }
}
