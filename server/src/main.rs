use crate::connection::handle_connection;
use common::Message;
use std::net::TcpListener;
use chrono::Local;

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

    // setup the listener
    let listener = match TcpListener::bind(&connection_info) {
        Ok(l) => l,
        Err(e) => {
            nay!("Error binding to {}: {}", connection_info, e);
            std::process::exit(1);
        }
    };

    // create the message history buffer
    let (server_channel, client_channel) = channel::ServerChannel::new();

    // start the listener
    for stream in listener.incoming() {
        // handle connection errors
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                nay!("Error accepting connection: {}", e);
                continue;
            }
        };

        server_channel.send(Message {
            message: "User has joined".to_owned(),
            author: "Server".to_owned(),
            timestamp: format!("{}", Local::now()),
        });

        let client_channel = client_channel.clone();

        // send the connection to a new thread
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, client_channel) {
                nay!("Error handling connection: {}", e);
            }
        });
    }
}
