use std::{error::Error, io::Read};

use common::message::Message;
use error::ServerError;
use hashbrown::HashMap;
use mio::{
    net::{TcpListener, TcpStream},
    Events, Interest, Poll, Token,
};

mod agent;
mod error;

pub struct Client {
    socket: TcpStream,
}

impl Client {
    fn read(&mut self) -> Result<Message, ServerError> {
        let mut size_bytes = [0u8; 4];
        self.socket.read_exact(&mut size_bytes)?;
        let size = u32::from_le_bytes(size_bytes);
        let mut data = vec![0u8; size as _];
        self.socket.read_exact(&mut data)?;

        Ok(bincode::deserialize(&data)?)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut poll = Poll::new()?;
    let mut events = Events::with_capacity(128);
    let mut listener = TcpListener::bind("127.0.0.1:39093".parse()?)?;
    let listener_token = Token(0);
    let mut clients = HashMap::new();
    let mut id_pool = 1;

    poll.registry()
        .register(&mut listener, listener_token, Interest::READABLE)?;

    loop {
        poll.poll(&mut events, None)?;

        for event in events.iter() {
            let tk = event.token();

            if listener_token == event.token() {
                match listener.accept() {
                    Ok((mut socket, addr)) => {
                        let id = id_pool;
                        let tk = Token(id);
                        id_pool += 1;

                        poll.registry()
                            .register(&mut socket, tk, Interest::READABLE)?;
                        let cl = Client { socket };
                        clients.insert(tk, cl);
                    }
                    Err(e) => {
                        println!("{e}");
                    }
                }
            } else {
                if let Some(cl) = clients.get_mut(&tk) {
                    match cl.read() {
                        Ok(msg) => {
                            println!("{msg}");
                        }
                        Err(e) => {
                            println!("{e}");
                        }
                    }
                }
            }
        }
    }
}
