use std::{
    path::PathBuf,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    time::{Duration, SystemTime},
};

use error::PackageError;
use rquickjs::{
    class::Trace,
    function::Args,
    loader::{BuiltinResolver, FileResolver, NativeLoader, ScriptLoader},
    CatchResultExt, Context, Ctx, Function, Module, Runtime, Value,
};

mod error;

#[derive(Trace, Clone)]
#[rquickjs::class]
pub struct App<M> {
    #[qjs(skip_trace)]
    service_tx: Sender<M>,
    #[qjs(skip_trace)]
    service_rx: Arc<Receiver<M>>,
}

impl<M> App<M> {
    pub fn sync_send(&self, msg: M) -> M {
        self.service_tx.send(msg).unwrap();
        self.service_rx.recv().unwrap()
    }
}

pub struct Package<M> {
    pub msg_tx: Sender<String>,
    pub service_rx: Receiver<M>,
    pub service_tx: Sender<M>,
}

fn init_package<'js, M>(
    c: Ctx<'js>,
    service_tx: Sender<M>,
    service_rx: Receiver<M>,
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

    let p = Module::evaluate(
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

    p.finish::<()>().catch(&c)?;

    Ok(())
}

fn run_package<F, M>(
    cb: F,
    root: PathBuf,
    msg_rx: Receiver<String>,
    service_tx: Sender<M>,
    service_rx: Receiver<M>,
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
        let print = Function::new(c.clone(), |args: Value| {
            println!("{:?}", args);
        })?;
        c.globals().set("print", print)?;
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

impl<M> Package<M>
where
    M: Send + 'static,
{
    pub fn load<F>(root: PathBuf, cb: F) -> Result<Package<M>, PackageError>
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
