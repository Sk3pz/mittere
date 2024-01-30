use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use common::Message;
use crate::connection::handle_connection;

mod config;
mod logging;
mod connection;

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
    let history: Arc<Mutex<Vec<Message>>> = Arc::new(Mutex::new(Vec::new()));

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

        // clone history to send to the client handler
        let history_clone = history.clone();

        // send the connection to a new thread
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, history_clone){
                nay!("Error handling connection: {}", e);
            }
        });
    }
}
