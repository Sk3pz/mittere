use send_it::async_reader::VarReader;
use send_it::async_writer::VarWriter;
use std::fmt::Display;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::net::tcp::{ReadHalf, WriteHalf};
use crate::channel::Channel;
use crate::{hey, say};
use common::message::{Message, MessageError};

#[derive(Debug)]
pub enum ClientError {
    IoError(std::io::Error),
    InvalidMessage(MessageError),
}

impl Display for ClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ClientError::IoError(e) => write!(f, "Client IO Error: {}", e),
            ClientError::InvalidMessage(e) => write!(f, "Invalid message: {}", e),
        }
    }
}

pub async fn handle_read_conn(mut read_stream: ReadHalf<'static>,
                              channel: Channel, username: String) -> Result<(), ClientError> {
    say!("Client {} connected.", username);
    channel.send(Message::new(format!("{} has connected.", username.clone()), "Server".to_string()));

    // reading data from the client
    let mut reader = VarReader::new(&mut read_stream);

    while let Ok(read) = reader.read_data().await {
        if read.len() != 1 {
            let _ = channel.send(Message::new("Invalid message".to_string(), "Server".to_string()));
            hey!("Invalid message: invalid segment count");
            return Err(ClientError::InvalidMessage(MessageError::InvalidSegmentCount));
        }

        // convert to a Message type
        let raw = read.first().unwrap().to_string();
        let message = Message::new(raw, username.to_string());

        say!("Message from {} @ {}: {}", message.author, message.timestamp, message.message);

        channel.send(message);
    }

    // send disconnect
    say!("Client {} disconnected.", username);
    channel.send(Message::new(format!("{} has disconnected.", username.clone()), "Server".to_string()));
    Ok(())
}

pub async fn handle_write_conn(mut write_stream: WriteHalf<'static>,
                               channel: Channel, username: String, should_close: Arc<AtomicBool>) -> Result<(), ClientError> {
    while !should_close.load(std::sync::atomic::Ordering::SeqCst) {
        // get data from main thread
        // this will hang the thread until a message is received, even if the socket is closed.
        let message = channel.receive();

        if message.author == username {
            continue;
        }

        // send the message
        // send the message to the client
        let mut writer = VarWriter::new();
        for segment in message.segmented() {
            writer.add(segment);
        }

        match writer.send(&mut write_stream).await {
            Ok(_) => {},
            Err(e) => {
                hey!("Error sending message: {}", e);
                return Err(ClientError::IoError(e));
            }
        }
    }

    Ok(())
}

