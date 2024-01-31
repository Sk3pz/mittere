use crate::connection::{ClientError, handle_read_conn, handle_write_conn};
use tokio::net::TcpListener;
use std::sync::Arc;
use send_it::async_reader::VarReader;
use tokio::runtime::Runtime;
use common::message::{Message, MessageError};

mod config;
mod connection;
mod logging;
mod channel;

#[tokio::main]
async fn main() {
    // get the config information
    // todo: make this a global constant and have it be platform specific for location
    let config = match config::Config::load("./config.toml") {
        Ok(c) => c,
        Err(e) => {
            nay!("Error loading config: {}", e);
            std::process::exit(1);
        }
    };

    let connection_info = config.conn;

    say!("Starting server");
    // setup the listener
    let listener = match TcpListener::bind(format!("{}:{}", connection_info.ip, connection_info.port)).await {
        Ok(l) => l,
        Err(e) => {
            nay!("Error binding to {}: {}", connection_info, e);
            std::process::exit(1);
        }
    };

    say!("Server started on {}. Creating channels and runtime.", connection_info);
    // create the message history buffer
    let client_channel = channel::Channel::new();

    say!("Server listening and accepting connections.");
    // start the listener
    loop {
        // get the stream
        let (mut stream, _) = match listener.accept().await {
            Ok(s) => s,
            Err(e) => {
                nay!("Error accepting connection: {}", e);
                continue;
            }
        };

        let client_channel = client_channel.clone();
        let read_client_channel = client_channel.clone();

        // get username from the client
        let mut reader = VarReader::new(&mut stream);
        let username = match reader.read_data().await.map_err(|e| ClientError::IoError(e))? {
            read if read.len() == 1 => read.first().unwrap().to_string(),
            _ => {
                let _ = client_channel.send(Message::new("Invalid message".to_string(), "Server".to_string()));
                hey!("Invalid message: invalid segment count");
                continue;
            }
        };

        let (read_stream, write_stream) = stream.split();

        let should_exit = Arc::new(false);

        // read handler
        tokio::spawn(async move {
            if let Err(e) = handle_read_conn(read_stream, read_client_channel, username.clone()).await {
                nay!("Error handling connection: {}", e);
            }
        });

        // write handler
        tokio::spawn(async move {
            if let Err(e) = handle_write_conn(write_stream, client_channel, username).await {
                nay!("Error handling connection: {}", e);
            }
        });
    }
}
