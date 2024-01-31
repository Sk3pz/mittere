use crate::connection::handle_connection;
use common::Message;
use std::net::TcpListener;
use std::sync::{mpsc, Arc, Mutex};

mod config;
mod connection;
mod logging;

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

    // cirx = client input receiver, citx = client input transmitter
    // citx is used to send messages to the main thread from the client threads
    // cirx is used to receive messages from the main thread to the client threads
    let (cirx, citx) = mpsc::channel::<Message>();
    let client_sender = Arc::new(Mutex::new(citx));
    // bcrx = broadcast receiver, bctx = broadcast transmitter
    // bctx is used to send messages to the client threads from the main thread
    // bcrx is used to receive messages from the client threads to the main thread
    let (bcrx, bctx) = mpsc::channel::<Message>();
    let client_receiver = Arc::new(Mutex::new(bcrx));

    let mut join_index = 0;

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

        client_receiver
            .lock()
            .unwrap()
            .send(Message {
                message: "User has joined".to_owned(),
                author: "User".to_owned(),
                timestamp: format!("{}", join_index),
            })
            .unwrap();
        join_index += 1;

        // clone client broadcaster and receiver
        let client_sender = Arc::clone(&client_sender);
        let client_receiver = Arc::clone(&client_receiver);

        // send the connection to a new thread
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, client_sender, client_receiver) {
                nay!("Error handling connection: {}", e);
            }
        });
    }
}
