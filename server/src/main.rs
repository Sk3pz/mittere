use crate::connection::handle_connection;
use std::net::TcpListener;
use std::sync::Arc;
use tokio::runtime::Runtime;
use crate::message::Message;

mod config;
mod connection;
mod logging;
mod channel;
pub mod message;

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
    let listener = match TcpListener::bind(&connection_info) {
        Ok(l) => l,
        Err(e) => {
            nay!("Error binding to {}: {}", connection_info, e);
            std::process::exit(1);
        }
    };

    say!("Server started on {}. Creating channels and runtime.", connection_info);
    // create the message history buffer
    let (server_channel, client_channel) = channel::ServerChannel::new();

    // create a runtime
    let runtime = match Runtime::new() {
        Ok(r) => r,
        Err(e) => {
            nay!("Error creating runtime: {}", e);
            return;
        }
    };

    // create an atomic runtime
    let runtime = Arc::new(runtime);

    say!("Server listening and accepting connections.");
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

        let client_channel = client_channel.clone();
        let runtime = runtime.clone();

        // send the connection to a new thread
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, runtime, client_channel).await {
                nay!("Error handling connection: {}", e);
            }
        });
    }
}
