// Mittere server

use std::net::{TcpListener, TcpStream};
use mittere_lib::logger::Logger;
use mittere_lib::make_logger;
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::command::command_processor;
use crate::client::{handle_client, check_disconnected};
use std::collections::HashMap;
use std::thread;

use mittere_lib::packet_capnp::{entry_point, entry_response, login, event};
use std::fs::File;
use chrono::Local;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use mittere_lib::network::entry_response_io::write_invalid_entry_response;
use std::ops::Add;

pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref global_logger: Arc<Mutex<Logger>> = Arc::new(Mutex::new(make_logger(true, true, true, false)));
    pub static ref connected_clients: Arc<Mutex<Vec<TcpStream>>> = Arc::new(Mutex::new(Vec::new())); // Why, God, do you let me do this?
    pub static ref connections: Arc<Mutex<usize>> = Arc::new(Mutex::new(0usize));
}

mod client;
mod command;

fn main() {
    // ==================== LOGGER ====================
    // Create a logger to output to console
    //let verbose = true;
    //let mut logger = make_logger(verbose, true, true, true);

    // ==================== CONFIGURATION ====================
    // The maximum connections the server can have at one time
    // TODO: Make configurable
    // default to 20 because that is within average usage (probably) (if they dont like it, then they can change it!)
    // -1 means 'infinite' (If they somehow have the ram for it, why not? :D)
    let max_connections = 20;

    // get config values
    // TODO: make these values be configurable
    let ip = "localhost"; // IP to listen on
    let port = "8080";    // Port to listen on
    let address = format!("{}:{}", ip, port);

    // TODO: can you guess what this needs to be? Configurable!
    let MOTD = String::from("==============================\nWelcome to the Mittere server!\n==============================");

    // ==================== COMMAND EXECUTION ====================
    // Start command execution thread
    let (cmd_sender, cmd_receiver): (Sender<String>, Receiver<String>) = channel();
    // only takes the sender because the command executor doesnt need data from the main thread, only needs to send data
    thread::spawn(move || command_processor(cmd_sender));

    // open the listener on the address
    let listener_result = TcpListener::bind(address.clone());
    if listener_result.is_err() {
        // TODO: give a better explanation as to what happened with more info to help the user fix the issue
        global_logger.lock().unwrap().failure(format!("Could not start TCP Listener on {}", address));
        return;
    }
    let listener = listener_result.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 01S");
    global_logger.lock().unwrap().info(format!("Started listening on {}.", address));

    // ==================== INCOMING CONNECTION REQUESTS ====================
    thread::spawn(move || {
        for stream in listener.incoming() {
            global_logger.lock().unwrap().info("Processing possible connection...");
            if stream.is_err() {
                global_logger.lock().unwrap().warn("Failed to accept client. Continuing to listen.");
                continue;
            }
            let s = stream.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 02S");

            let ip = s.peer_addr()
                .expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 03S")
                .ip().to_string();

            // this is scuffed...
            if (connections.lock().unwrap().gt(&0usize)) && (connections.lock().unwrap().gt(&max_connections)) {
                write_invalid_entry_response(&s, String::from("The server is full!"));

                global_logger.lock().unwrap().warn(format!("A connection was attempted by {} but was refused: server is full.", ip));
                drop(s);
                continue;
            }
            global_logger.lock().unwrap().info(format!("Accepted connection: {}.", ip));

            let handler_s = s.try_clone();

            if handler_s.is_err() {
                // TODO: send error entry response
                global_logger.lock().unwrap().warn("Failed to clone connection for client handler, Not handling last connection.");
                drop(s);
                continue;
            }

            connected_clients.lock().unwrap().push(s);

            let motd_clone = MOTD.clone();

            thread::spawn(move || handle_client(handler_s.unwrap(), motd_clone));

            // still scuffed...
            connections.lock().unwrap().add(1usize);
        }
    });

    // ==================== MAIN LOOP ====================
    loop {
        // ==================== COMMAND PROCESSOR DATA PROCESSING ====================
        // at top of loop so incoming connections can be processed more spread out instead of
        // handle incoming connections, then do everything else, then repeat
        for s in &cmd_receiver {
            // TODO: process command data
        }
    }
}
