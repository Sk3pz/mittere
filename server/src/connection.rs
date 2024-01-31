use common::Message;
use send_it::reader::VarReader;
use send_it::writer::VarWriter;
use std::fmt::Display;
use std::net::TcpStream;
use std::sync::mpsc::{Receiver, Sender};
use std::sync::{Arc, Mutex};
use crate::channel::ClientChannel;

use crate::say;

#[derive(Debug)]
pub enum ClientError {
    IoError(std::io::Error),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClientError::IoError(e) => write!(f, "Client IO Error: {}", e),
        }
    }
}

pub fn handle_connection(mut stream: TcpStream, channel: ClientChannel) -> Result<(), ClientError> {
    let mut reader = VarReader::new(&mut stream);
    let mut writer = VarWriter::new();

    {
        let recv = receiver.lock().unwrap();
        let msg = recv.recv().unwrap();
        say!("{}", msg.message);
    }

    Ok(())
}

