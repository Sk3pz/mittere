// Mittere server

use std::net::{TcpListener, TcpStream};
use mittere_lib::logger::Logger;
use mittere_lib::make_logger;
use std::sync::mpsc::{Sender, Receiver, channel};
use threadpool::ThreadPool;
use crate::command::command_processor;
use crate::client_link::client_link;
use crate::client::handle_client;
use std::collections::HashMap;

mod client;
mod command;

fn main() {
    // ==================== LOGGER ====================
    // Create a logger to output to console
    let mut logger = make_logger(true, true, true, true);

    // ==================== CONFIGURATION ====================
    // The maximum connections the server can have at one time
    // TODO: Make configurable
    let max_connections = 8;

    // get config values
    // TODO: make these values be configurable
    let ip = "localhost"; // IP to listen on
    let port = "8080";    // Port to listen on
    let address = format!("{}:{}", ip, port);

    // ==================== THREAD POOL ====================
    // Create a thread pool to spawn new processes in
    let n_workers = max_connections + 1; // plus 1 for the command_processor
    let pool = ThreadPool::new(n_workers);

    // ==================== COMMAND EXECUTION ====================
    // Start command execution thread
    let (cmd_sender, cmd_receiver): (Sender<String>, Receiver<String>) = channel();
    // only takes the sender because the command executer doesnt need data from the main thread, only needs to send data
    pool.execute(move || command_processor(cmd_sender));


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
    let mut clients: Vec<TcpStream> = Vec::new(); // TODO: maybe have system that runs client_handler one time per loop in another thread?

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
            if current_connections > max_connections {
                // TODO: send SERVER_FULL packet
                // TODO: queue list?
                logger.warn(format!("A connection was attempted by {} but was refused: server is full.",
                                    s.local_addr().expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 03Sa")));
                drop(s);
                continue;
            }
            logger.info(format!("Accepted connection: {}.", s.local_addr().expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 03Sb")));

            let handler_s = s.try_clone();

            if handler_s.is_err() {
                // TODO: send error packet
                logger.warn("Failed to clone connection for client handler, Not handling last connection.");
                drop(s);
                continue;
            }

            clients.push(s);
            let c_sender = client_sender.clone();
            pool.execute(move || handle_client(handler_s.unwrap(), c_sender.clone()));
        }

        // ==================== CLIENT HANDLING ====================
        // handle currently connected users
        for s in &receiver {
            // TODO: handle clients
        }
    }

    drop(listener); // close all connections that still remain
}
