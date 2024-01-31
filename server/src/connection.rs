use send_it::reader::VarReader;
use send_it::writer::VarWriter;
use std::fmt::Display;
use std::net::TcpStream;
use std::sync::Arc;
use tokio::runtime::Runtime;
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

pub async fn handle_connection(mut stream: TcpStream, runtime: Arc<Runtime>, channel: Channel) -> Result<(), ClientError> {

    // copy the stream
    let mut read_stream = stream.try_clone().map_err(|e| ClientError::IoError(e))?;
    let mut writer_channel = channel.clone();

    // get username from the client
    let mut reader = VarReader::new(&mut read_stream);
    let username = match reader.read_data().map_err(|e| ClientError::IoError(e))? {
        read if read.len() == 1 => read.first().unwrap().to_string(),
        _ => {
            let _ = channel.send(Message::new("Invalid message".to_string(), "Server".to_string()));
            hey!("Invalid message: invalid segment count");
            return Err(ClientError::InvalidMessage(MessageError::InvalidSegmentCount));
        }
    };

    say!("Client {} connected.", username);
    channel.send(Message::new(format!("{} has connected.", username.clone()), "Server".to_string()));
    let author = username.clone();

    // handling messages from other clients
    let writer = runtime.spawn(async move {
        loop {
            // get data from main thread
            // this will hang the thread until a message is received, even if the socket is closed.
            let message = writer_channel.receive();

            if message.author == author {
                continue;
            }

            // send the message to the client
            let mut writer = VarWriter::new();
            for segment in message.segmented() {
                writer.add(segment);
            }

            match writer.send(&mut stream) {
                Ok(_) => {},
                Err(e) => {
                    hey!("Error sending message: {}", e);
                }
            }
        }
    });

    // reading data from the client
    let mut reader = VarReader::new(&mut read_stream);

    while let Ok(read) = reader.read_data() {
        if read.len() != 1 {
            let _ = channel.send(Message::new("Invalid message".to_string(), "Server".to_string()));
            hey!("Invalid message: invalid segment count");
            break; // this currently is set up to close the connection if the message is invalid
        }

        // convert to a Message type
        let raw = read.first().unwrap().to_string();
        let message = Message::new(raw, username.to_string());

        say!("Message from {} @ {}: {}", message.author, message.timestamp, message.message);

        channel.send(message);
    }

    writer.abort();

    // send disconnect message
    say!("Client {} disconnected.", username);
    channel.send(Message::new(format!("{} has disconnected.", username.clone()), "Server".to_string()));

    Ok(())
}

