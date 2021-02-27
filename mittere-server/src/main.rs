// Mittere server

use std::net::{TcpListener, TcpStream};
use mittere_lib::logger::Logger;
use mittere_lib::{make_logger, unwrap_or_default};
use std::sync::mpsc::{Sender, Receiver, channel};
use crate::command::command_processor;
use crate::client::handle_client;
use std::collections::HashMap;
use std::{thread, fs};

use mittere_lib::packet_capnp::{entry_point, entry_response, login, event};
use std::fs::File;
use chrono::Local;
use serde::Deserialize;

use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use mittere_lib::network::entry_response_io::write_invalid_entry_response;
use std::ops::Add;
use uuid::Uuid;
use std::path::Path;
use std::io::Read;
use crate::file::read_config;

pub const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref global_logger: Arc<Mutex<Logger>> = Arc::new(Mutex::new(make_logger(true, true, true, false)));
    pub static ref connected_clients: Arc<Mutex<HashMap<Uuid, TcpStream>>> = Arc::new(Mutex::new(HashMap::new()));
    pub static ref connections: Arc<Mutex<usize>> = Arc::new(Mutex::new(0usize)); // TODO: rework
}

mod client;
mod command;
mod file;

fn main() {
    // ==================== Config Files ====================
    let cdir = std::env::current_dir().expect("Error in attempting to get config file: no access.");
    let current_dir = cdir.as_path().to_str().expect("Error in attempting to get config file: no access.");
    let config_path = format!("{}/mittere-config/server-config.toml", current_dir);
    let raw_path = Path::new(&config_path);

    let mut config = read_config(raw_path, String::from("# This is an example configuration for the Mittere server\
    \n
    \n# The general section is for misc values\
    \n[general]\
    \n# connections: the maximum amount of clients that can connect at 1 time\
    \n# -1 = infinite amount (Not recommended unless you have a supercomputer!)\
    \nconnections = -1\
    \n# Motd: the message of the day that clients will see when first connecting\
    \n# uses ANSI color codes (If you dont know what this means, dont use it! A better color reading system is coming soon!)\
    \nmotd = \"==============================\\nWelcome to the Mittere server!\\n==============================\"
    \n\n# The values for the server\
    \n[server]\
    \n# ip: the ip to listen on\
    \n# defaults to 0.0.0.0 and will listen on your machines current IP\
    \nip = \"0.0.0.0\"
    \n# port: the port to listen on\
    \n# defaults to 2277\
    \nport = \"2277\""));

    let general_config = config.general.expect("Could not find [general] in config!");
    let server_config = config.server.expect("Could not find [server] in config!");

    // ==================== CONFIGURATION ====================
    // The maximum connections the server can have at one time
    // TODO: Make configurable
    // default to 20 because that is within average usage (probably) (if they dont like it, then they can change it!)
    // -1 means 'infinite' (If they somehow have the ram for it, why not? :D)
    let max_connections_cfg = general_config.connections.expect("Could not find general.max-connections in the config!");
    let infi_conn = max_connections_cfg < 0;
    let max_connections = if max_connections_cfg > 0 { max_connections_cfg as usize } else { 0 };

    // get config values
    // TODO: make these values be configurable
    let ip = unwrap_or_default(server_config.ip, String::from("0.0.0.0")); // IP to listen on
    let port = unwrap_or_default(server_config.port, String::from("2277"));    // Port to listen on
    let address = format!("{}:{}", ip, port);

    // TODO: make this configurable in 'motd.txt'
    let MOTD = unwrap_or_default(general_config.motd, String::from("Welcome to the Mittere server!"));

    // ==================== COMMAND EXECUTION ====================
    // Start command execution thread
    let (cmd_sender, cmd_receiver): (Sender<String>, Receiver<String>) = channel();
    // only takes the sender because the command executor doesnt need data from the main thread, only needs to send data
    thread::spawn(move || command_processor(cmd_sender));

    // open the listener on the address
    let listener_result = TcpListener::bind(address.clone());
    if listener_result.is_err() {
        // TODO: give a better explanation as to what happened with more info to help the user fix the issue?
        global_logger.lock().unwrap().failure(format!("Could not start TCP Listener on {}", address));
        return;
    }
    let listener = listener_result.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 01S");
    global_logger.lock().unwrap().info(format!("Started listening on {}.", address));

    // ==================== INCOMING CONNECTION REQUESTS ====================
    thread::spawn(move || {
        for stream in listener.incoming() {
            global_logger.lock().unwrap().verbose("Processing possible connection...");
            if stream.is_err() {
                global_logger.lock().unwrap().warn("Failed to accept client. Continuing to listen.");
                continue;
            }
            let s = stream.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 02S");

            let ip = s.peer_addr()
                .expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 03S")
                .ip().to_string();

            // this is scuffed...
            if (!infi_conn) || ((connections.lock().unwrap().gt(&0usize)) && (connections.lock().unwrap().gt(&max_connections))) {
                write_invalid_entry_response(&s, String::from("The server is full!"));

                global_logger.lock().unwrap().warn(format!("A connection was attempted by {} but was refused: server is full.", ip));
                drop(s);
                continue;
            }
            global_logger.lock().unwrap().verbose(format!("Accepted connection: {}.", ip));

            let handler_s = s.try_clone();

            if handler_s.is_err() {
                global_logger.lock().unwrap().warn("Failed to clone connection for client handler, Not handling last connection.");
                // send an entry response for the client to read
                write_invalid_entry_response(&s, String::from("A connection error occurred on the server. If this continues, the server may need to be restarted."));
                drop(s);
                continue;
            }

            let uuid = Uuid::new_v4();

            connected_clients.lock().unwrap().insert(uuid, s);

            let motd_clone = MOTD.clone();

            thread::spawn(move || handle_client(handler_s.unwrap(), uuid, motd_clone));

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
