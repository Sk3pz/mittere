use send_it::async_reader::VarReader;
use send_it::async_writer::VarWriter;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use crate::channel::AtomicChannel;
use crate::{Event, hey, say};
use common::message::{Message, MessageError};
use common::to_local_time;
use crate::id_allocator::IdAllocator;

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
                              id: usize, log_msgs: bool) -> Result<(), ClientError> {
    let mut reader = VarReader::new(&mut read_stream);
    // get username from the client
    // todo: handle login here
    let username = match reader.read_data().await {
        Ok(read) if read.len() == 1 => read.first().unwrap().to_string(),
        _ => {
            hey!("Invalid message: invalid segment count");
            return Err(ClientError::InvalidMessage(MessageError::InvalidSegmentCount));
        }
    };
    channel.send(Event::Login { id, username: username.clone() });

    say!("Client {} connected.", username);
    channel.send(Event::Message(
        Message::new(format!("{} has connected.", username.clone()), "Server".to_string())));

    // reading data from the client
    let mut reader = VarReader::new(&mut read_stream);

    while let Ok(read) = reader.read_data().await {
        if read.len() != 1 {
            let _ = channel.send(Event::Message(
                Message::new("Invalid message".to_string(), "Server".to_string())));
            // most likely disconnecting
            return Ok(())
        }

        // convert to a Message type
        let raw = read.first().unwrap().to_string();
        let message = Message::new(raw, username.to_string());

        if log_msgs {
            let local_time = to_local_time(message.timestamp.clone()).unwrap();
            say!("{} {} ({}): {}", local_time.format("%m/%d/%Y %I:%M%p"), message.author, id, message.message);
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
                               channel: AtomicChannel<Event>,
                               id: usize, id_allocator: Arc<Mutex<IdAllocator>>) -> Result<(), ClientError> {
    let username: String;
    loop {
        let msg = channel.receive();
        match msg {
            Event::Login { id: to_login_id, username: received_username } if to_login_id == id => {
                username = received_username;
                break;
            },
            Event::Disconnect { id: to_disconnect_id } if to_disconnect_id == id => {
                id_allocator.lock().unwrap().free(id);
                return Ok(());
            },
            Event::Shutdown => {
                id_allocator.lock().unwrap().free(id);
                return Ok(());
            },
            _ => continue,
        }
    }
    // todo: clients not receiving some messages
    hey!("{} Starting write loop", id);
    loop {
        // get data from main thread
        // this will hang the thread until a message is received, even if the socket is closed.
        let message = match channel.receive() {
            Event::Login {..} => continue, // ignore logins
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
                // todo: better error handling here
                // id_allocator.lock().unwrap().free(id);
                // return Err(ClientError::IoError(e));
            }
        }
    }

    id_allocator.lock().unwrap().free(id);
    Ok(())
}

