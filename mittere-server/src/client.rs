use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use mittere_lib::network::entry_point_io::read_entry_point;
use mittere_lib::logger::Logger;
use mittere_lib::network::login_data::LoginData;
use crate::SERVER_VERSION;
use mittere_lib::network::entry_response_io::{write_ping_entry_response, write_valid_entry_response, write_invalid_entry_response};
use mittere_lib::network::event_io::{read_event, write_event_error, write_event_keepalive, write_event_message};
use crate::{global_logger, connected_clients, connections};
use std::ops::Sub;
use mittere_lib::{systime, KEEPALIVE_INTERVAL, to_epoch};
use chrono::Duration;
use std::time::SystemTime;
use better_term::style::Color;
use uuid::Uuid;

pub fn send_to_clients(msg: String) {
    let clients = connected_clients.lock().unwrap();
    for c in clients.values() {
        write_event_message(c, msg.clone());
    }
}

/// Properly send all the disconnect messages when a client disconnects
/// used to remove duplicate code
pub fn disconnect_client(uuid: Uuid, username: String) {
    global_logger.lock().unwrap().info(format!("Client {} has disconnected", username));
    send_to_clients(format!("{}{} has disconnected.", Color::BrightYellow, username));
    connected_clients.lock().unwrap().remove(&uuid);
    if connections.lock().unwrap() > 0 {
        connections.lock().unwrap().sub(1usize);
    } else {
        global_logger.lock().unwrap().warn("User disconnected when there were no clients connected: The maximum connections value will not work properly.");
    }
}

pub fn handle_client(stream: TcpStream, uuid: Uuid, motd: String) {

    // handle EntryPoint
    let (login, version, err) = read_entry_point(&stream);
    if version.is_some() { // just a 'ping' to check the server is compatible
        let server_version = String::from(SERVER_VERSION);
        write_ping_entry_response(&stream, server_version == version.unwrap(), server_version);
        return;
    }
    if err.is_some() { // something went wrong
        global_logger.lock().unwrap().error(format!("Encountered an error reading entry point for client {}:\n{}", stream.peer_addr().unwrap().ip().to_string(), err.unwrap()));
        write_invalid_entry_response(&stream, String::from("Failed to read entry point: invalid!"));
        return;
    }
    if !login.is_some() { // actual login attempt
        return;
    }

    // Send login response and process data
    let l: LoginData = login.unwrap();
    // TODO: validate login attempt & store username for use (password not stored in memory)
    let username = l.username;
    write_valid_entry_response(&stream, motd); // TODO: remove. for now, just send a valid entry response to ensure that testing works
    global_logger.lock().unwrap().info(format!("Client {} has connected.", username));
    // TODO: send raw join message to all clients

    let mut last_keepalive = SystemTime::now();
    let mut expecting_keepalive = false;
    let mut ping = 0;

    // get client data from client file
    let mut display_name = username.clone();
    let mut display_color = format!("{}", Color::White);
    let mut message_color = format!("{}", Color::White);

    // main client connection loop: Process Event packets
    loop {
        // disconnect flag
        let mut disconnect = false;

        // ensure the connection is still open with keepalive
        let now = SystemTime::now();
        let duration = now.duration_since(last_keepalive).expect("Fatal error occurred: System time moved backwards! Are you a time traveler?")
            .as_secs();
        if duration >= KEEPALIVE_INTERVAL {
            if !expecting_keepalive { // if there is not a keepalive expected, send a request
                write_event_keepalive(&stream);
                last_keepalive = SystemTime::now();
                expecting_keepalive = true;
            } else { // if there is a keepalive scheduled, disconnect the client
                disconnect_client(uuid, username);
                return;
            }
        }

        // listen for event packet
        let (msg, cfg, time, error, disconnect_status) = read_event(&stream);
        disconnect = disconnect_status;
        if msg.is_some() {
            // The client has sent a message and should be sent to other clients
            let message = msg.unwrap().to_string();

            let formatted = format!("{}{} {}> {}{}", display_color.clone(), display_name.clone(), Color::BrightBlack, message_color.clone(), message.clone());
            // TODO: remove injection attack? (or leave it for first release because why not >:) )

            global_logger.lock().unwrap().chat(message, message_color.clone(), display_name.clone(), display_color.clone());
            send_to_clients(formatted);
        } else if time.is_some() {
            if !expecting_keepalive {
                // Not expecting a keepalive, ignore
                continue;
            }
            let client_time = time.unwrap();
            ping = (client_time) - to_epoch(last_keepalive).as_secs();
            expecting_keepalive = false;
        } else if cfg.is_some() {
            // TODO: update config file
            let cfg_ = cfg.unwrap();
            display_name = cfg_.display_name;
            display_color = cfg_.name_color;
            message_color = cfg_.msg_color;
        } else {
            if error.is_some() { // if the client sent an error message, log it in the server (maybe implement error handling later?)
                let err = error.unwrap();
                global_logger.lock().unwrap().error(format!("Client {} has sent an error event: {}", username, err));
            } else { // if the client didnt send an error, then an invalid packet was sent.
                global_logger.lock().unwrap().warn(format!("Client {} has sent an invalid packet, most likely a forced disconnect. They have been disconnected.", username));
            }

            disconnect = true;
        }

        if disconnect { // if the disconnect flag is set, disconnect the client
            disconnect_client(uuid, username);
            drop(stream); // not needed, but for clarity
            return;
        }

    }

}