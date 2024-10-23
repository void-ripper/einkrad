use std::{
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
};

use error::PackageError;
pub use message::Message;
use rquickjs::{
    class::Trace,
    loader::{FileResolver, ScriptLoader},
    CatchResultExt, Context, Ctx, Function, Runtime,
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
    c.eval::<(), _>(
        r#"
                import {onStart, onUpdate, onMessage, name} from "./index.js";
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
        FileResolver::default().with_path(root.to_string_lossy().to_string()),
        ScriptLoader::default(),
    );
    let ctx = Context::base(&rt)?;

    ctx.with(|c| {
        cb(c.clone());
        init_package(c, service_tx, service_rx)
    })?;

    loop {
        ctx.with(|c| {
            let on_message: Function = c.globals().get("onMessage").unwrap();
            while let Ok(msg) = msg_rx.try_recv() {
                let ret: Result<(), _> = on_message.call((msg,)).catch(&c);
                if let Err(e) = ret {
                    println!("{e}");
                }
            }

            let _: () = c.eval("onUpdate()").unwrap();
        });
    }
}

impl Package {
    pub fn load<F>(root: PathBuf, cb: F) -> Result<Package, PackageError>
    where
        F: Fn(Ctx) + Send + 'static,
    {
        println!("Package: try to load {}", root.display());
        let indexjs = root.join("index.js");

        if indexjs.exists() {
            return Err(PackageError::not_a_package());
        }

        let (tx, rx) = mpsc::channel();
        let (rtx, rrx) = mpsc::channel();
        let (atx, arx) = mpsc::channel();

        std::thread::spawn(|| {
            if let Err(e) = run_package(cb, root, rx, rtx, arx) {
                println!("Package: {e}");
            }
        });

        Ok(Self {
            msg_tx: tx,
            service_tx: atx,
            service_rx: rrx,
        })
    }
}
