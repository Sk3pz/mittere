use send_it::async_reader::VarReader;
use send_it::async_writer::VarWriter;
use std::fmt::Display;
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use crate::channel::AtomicChannel;
use crate::{Event, hey, say};
use common::message::{Message, MessageError};
use common::to_local_time;

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

pub async fn handle_read_conn(mut read_stream: OwnedReadHalf, channel: AtomicChannel<Event>,
                              username: String, id: usize, log_msgs: bool) -> Result<(), ClientError> {
    say!("Client {} connected.", username);
    channel.send(Event::Message(
        Message::new(format!("{} has connected.", username.clone()), "Server".to_string())));

    // reading data from the client
    let mut reader = VarReader::new(&mut read_stream);

    while let Ok(read) = reader.read_data().await {
        if read.len() != 1 {
            let _ = channel.send(Event::Message(
                Message::new("Invalid message".to_string(), "Server".to_string())));
            hey!("Invalid message: invalid segment count");
            return Err(ClientError::InvalidMessage(MessageError::InvalidSegmentCount));
        }

        // convert to a Message type
        let raw = read.first().unwrap().to_string();
        let message = Message::new(raw, username.to_string());

        if log_msgs {
            let local_time = to_local_time(message.timestamp.clone()).unwrap();
            say!("{} {}: {}", local_time.format("%m/%d/%Y %I:%M%p"), message.author, message.message);
        }

        channel.send(Event::Message(message));
    }

    // send disconnect to write thread
    channel.send(Event::Disconnect { id });

    // notify disconnect
    say!("Client {} disconnected.", username);
    channel.send(Event::Message(Message::new(format!("{} has disconnected.", username.clone()), "Server".to_string())));
    Ok(())
}

pub async fn handle_write_conn(mut write_stream: OwnedWriteHalf,
                               channel: AtomicChannel<Event>, username: String, id: usize) -> Result<(), ClientError> {
    loop {
        // get data from main thread
        // this will hang the thread until a message is received, even if the socket is closed.
        let message = match channel.receive() {
            // receive the message
            Event::Message(msg) => msg,
            // disconnect if requested
            Event::Disconnect { id: to_disconnect_id } if to_disconnect_id == id => break,
            // skip over disconnects for other clients
            Event::Disconnect { .. } => continue,
            // handle server shutdown
            Event::Shutdown => break
        };

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

