use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use crate::connection::{handle_read_conn, handle_write_conn};
use tokio::net::TcpListener;
use common::message::Message;
use crate::channel::AtomicChannel;

mod config;
mod connection;
mod logging;
mod channel;
mod id_allocator;

#[derive(Debug, Clone)]
pub enum Event {
    Login { id: usize, username: String },
    Message(Message),
    Disconnect { id: usize },
    Shutdown,
}

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
    // set up the listener
    let listener = match TcpListener::bind(format!("{}:{}", connection_info.ip, connection_info.port)).await {
        Ok(l) => l,
        Err(e) => {
            nay!("Error binding to {}: {}", connection_info, e);
            std::process::exit(1);
        }
    };

    say!("Server started on {}. Creating channels and runtime.", connection_info);
    // create the message history buffer
    let client_channel: AtomicChannel<Event> = AtomicChannel::new();

    let id_allocator = Arc::new(Mutex::new(id_allocator::IdAllocator::new()));

    // handle ctrl+c
    let listening = Arc::new(AtomicBool::new(true));

    let ctrl_c_listening = listening.clone();
    tokio::spawn(async move {
        if let Err(e) = tokio::signal::ctrl_c().await {
            nay!("Error handling ctrl+c: {}", e);
        }
        ctrl_c_listening.store(false, Ordering::SeqCst);
    });

    say!("Server listening and accepting connections.");
    // start the listener
    while listening.load(Ordering::SeqCst) {
        // get the stream
        // todo: this timeout could cause issues, needs further testing!
        let incoming = tokio::time::timeout(Duration::from_millis(5000), listener.accept()).await;
        if let Err(_) = incoming {
            continue;
        }
        let (stream, _) = match incoming.unwrap() {
            Ok(s) => s,
            Err(e) => {
                nay!("Error accepting connection: {}", e);
                continue;
            }
        };

        let client_channel = client_channel.clone();
        let read_client_channel = client_channel.clone();

        // determine the id of the client
        let id = id_allocator.lock().unwrap().allocate();

        let id_allocator_clone = id_allocator.clone();

        // split the stream into read and write
        let (read_stream, write_stream) = stream.into_split();

        let reader_id = id.clone();

        let log_msgs = config.general.show_msgs_on_server.clone();

        // spawn the read handler
        tokio::spawn(async move {
            if let Err(e) = handle_read_conn(read_stream, read_client_channel, reader_id, log_msgs).await {
                nay!("Error handling connection: {}", e);
            }
        });

        // spawn the write handler
        tokio::spawn(async move {
            if let Err(e) = handle_write_conn(write_stream, client_channel, id, id_allocator_clone).await {
                nay!("Error handling connection: {}", e);
            }
        });
    }

    client_channel.send(Event::Shutdown);
}
