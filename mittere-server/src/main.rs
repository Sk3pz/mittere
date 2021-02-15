// Mittere server

use std::net::{TcpListener, TcpStream};
use mittere_lib::logger::Logger;
use mittere_lib::make_logger;
use std::sync::mpsc::{Sender, Receiver, channel};
use threadpool::ThreadPool;
use crate::command::command_processor;
use crate::client::handle_client;
use std::collections::HashMap;
use std::thread;

use mittere_lib::packet_capnp::{entry_point, entry_response, login, config_data, event};
use std::fs::File;

mod client;
mod command;

fn main() {
    let verbose = true;

    // ==================== LOGGER ====================
    // Create a logger to output to console
    let mut logger = make_logger(verbose, true, true, true, None);

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

    // ==================== COMMAND EXECUTION ====================
    // Start command execution thread
    let (cmd_sender, cmd_receiver): (Sender<String>, Receiver<String>) = channel();
    // only takes the sender because the command executor doesnt need data from the main thread, only needs to send data
    thread::spawn(move || command_processor(cmd_sender));

    // ==================== CONNECTION PROCESSES ====================
    // how many connections there are
    let mut current_connections = 0;

    // open the listener on the address
    let listener_result = TcpListener::bind(address.clone());
    if listener_result.is_err() {
        // TODO: give a better explanation as to what happened with more info to help the user fix the issue
        logger.failure(format!("Could not start TCP Listener on {}", address));
        return;
    }
    let listener = listener_result.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 01S");
    logger.info(format!("Started listening on {}.", address));

    // ==================== CLIENT HANDLING STRUCTURES ====================
    // Channel for the clients to send data to the server (All clients sent data will be put into receiver)
    let (client_sender, receiver): (Sender<String>, Receiver<String>) = channel();
    // store all connected clients
    let mut clients: Vec<TcpStream> = Vec::new();

    // ==================== MAIN LOOP ====================
    loop {
        // ==================== COMMAND PROCESSOR DATA PROCESSING ====================
        // at top of loop so incoming connections can be processed more spread out instead of
        // handle incoming connections, then do everything else, then repeat
        for s in &cmd_receiver {
            // TODO: process command data
        }

        // ==================== INCOMING CONNECTION REQUESTS ====================
        for stream in listener.incoming() {
            logger.info("Processing possible connection...");
            if stream.is_err() {
                logger.warn("Failed to accept client. Continuing to listen.");
                continue;
            }
            let s = stream.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 02S");

            let ip = s.peer_addr()
                .expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 03S")
                .ip().to_string();

            // I am not sure why I allow the maximum users setting to be 0 if the user sets it but whatever :)
            if (current_connections >= 0) && (current_connections > max_connections) {
                // TODO: Send LoginValidate with 'The server is full!' as the error
                let message = capnp::message::Builder::new_default();

                logger.warn(format!("A connection was attempted by {} but was refused: server is full.", ip));
                drop(s);
                continue;
            }
            logger.info(format!("Accepted connection: {}.", ip));

            let handler_s = s.try_clone();

            if handler_s.is_err() {
                // TODO: send error packet
                logger.warn("Failed to clone connection for client handler, Not handling last connection.");
                drop(s);
                continue;
            }

            clients.push(s);
            let c_sender = client_sender.clone();

            thread::spawn(move || handle_client(handler_s.unwrap(),
                                                make_logger(verbose, true,
                                                            true, false, Some(ip)),
                                                c_sender.clone()));
        }

        // ==================== CLIENT HANDLING ====================
        // handle currently connected users
        for s in &receiver {
            // TODO: handle clients
        }
    }

    drop(listener); // close all connections that still remain
}
