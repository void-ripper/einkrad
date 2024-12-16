use std::{error::Error, io::Read};

use common::message::Message;
use error::ServerError;
use hashbrown::HashMap;
use tokio::net::TcpListener;

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut listener = TcpListener::bind(([127, 0, 0, 1], 39093));
    let mut clients = HashMap::new();
    let mut id_pool = 1;

    loop {
        // let (sck, addr) = listener.ac
    }
}
