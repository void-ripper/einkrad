use std::{
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    time::{Duration, SystemTime},
};

use error::PackageError;
pub use message::Message;
use rquickjs::{
    class::Trace,
    loader::{BuiltinResolver, FileResolver, NativeLoader, ScriptLoader},
    CatchResultExt, Context, Ctx, Function, Module, Runtime,
};

mod error;
mod message;

#[derive(Trace, Clone)]
#[rquickjs::class]
pub struct App {
    #[qjs(skip_trace)]
    service_tx: Sender<Message>,
    #[qjs(skip_trace)]
    service_rx: Arc<Receiver<Message>>,
}

impl App {
    pub fn sync_send(&self, msg: Message) -> Message {
        self.service_tx.send(msg).unwrap();
        self.service_rx.recv().unwrap()
    }
}

pub struct Package {
    pub msg_tx: Sender<String>,
    pub service_rx: Receiver<Message>,
    pub service_tx: Sender<Message>,
}

fn init_package<'js>(
    c: Ctx<'js>,
    service_tx: Sender<Message>,
    service_rx: Receiver<Message>,
) -> Result<(), PackageError> {
    c.globals()
        .set(
            "App",
            App {
                service_tx,
                service_rx: Arc::new(service_rx),
            },
        )
        .catch(&c)?;

    Module::evaluate(
        c.clone(),
        "main",
        r#"
                import {onStart, onUpdate, onMessage, name} from "index.js";
                globalThis.name = name;
                globalThis.onUpdate = onUpdate;
                globalThis.onMessage = onMessage;

                onStart();
            "#,
    )
    .catch(&c)?;

    Ok(())
}

fn run_package<F>(
    cb: F,
    root: PathBuf,
    msg_rx: Receiver<String>,
    service_tx: Sender<Message>,
    service_rx: Receiver<Message>,
) -> Result<(), PackageError>
where
    F: Fn(Ctx),
{
    let rt = Runtime::new()?;

    rt.set_loader(
        (
            FileResolver::default()
                .with_path(&root.to_string_lossy())
                .with_native(),
            BuiltinResolver::default(),
        ),
        (ScriptLoader::default(), NativeLoader::default()),
    );
    // BUG: base should work but crashes!
    // let ctx = Context::base(&rt)?;
    let ctx = Context::full(&rt)?;

    ctx.with(|c| {
        cb(c.clone());
        init_package(c, service_tx, service_rx)
    })?;

    loop {
        let start = SystemTime::now();

        ctx.with(|c| -> Result<(), PackageError> {
            let on_message: Function = c.globals().get("onMessage").catch(&c)?;
            while let Ok(msg) = msg_rx.try_recv() {
                let _: () = on_message.call((msg,)).catch(&c)?;
            }

            let _: () = c.eval("onUpdate()").catch(&c)?;

            Ok(())
        })?;

        let elapsed = start.elapsed().unwrap();
        let to_wait = Duration::from_millis(100) - elapsed;
        std::thread::sleep(to_wait);
    }
}

impl Package {
    pub fn load<F>(root: PathBuf, cb: F) -> Result<Package, PackageError>
    where
        F: Fn(Ctx) + Send + 'static,
    {
        println!("PACKAGE: load {}", root.display());
        let indexjs = root.join("index.js");

        if !indexjs.exists() {
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
