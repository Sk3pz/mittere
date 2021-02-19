use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use mittere_lib::network::entry_point_io::read_entry_point;
use mittere_lib::logger::Logger;
use mittere_lib::network::login_data::LoginData;
use crate::SERVER_VERSION;
use mittere_lib::network::entry_response_io::{write_ping_entry_response, write_valid_entry_response, write_invalid_entry_response};
use mittere_lib::network::event_io::{read_event, write_event_error, write_event_keepalive};
use crate::{global_logger, connected_clients, connections};
use std::ops::Sub;
use mittere_lib::{systime, KEEPALIVE_INTERVAL};
use chrono::Duration;
use std::time::SystemTime;

/// Properly send all the disconnect messages when a client disconnects
/// used to remove duplicate code
pub fn disconnect_client(username: String) {
    // TODO: send raw disconnect message to all users
    global_logger.lock().unwrap().info(format!("Client {} has disconnected", username));
}

pub fn handle_client(stream: TcpStream, motd: String) {

    global_logger.lock().unwrap().important("A new client has appeared!");

    // handle EntryPoint
    let (login, version, err) = read_entry_point(&stream);
    if version.is_some() { // just a 'ping' to check the server is compatible
        let server_version = String::from(SERVER_VERSION);
        write_ping_entry_response(&stream, server_version == version.unwrap(), server_version);
        disconnect_client(username);
        return;
    }
    if err.is_some() { // something went wrong
        global_logger.lock().unwrap().error(format!("Encountered an error reading entry point for client {}:\n{}", stream.peer_addr().unwrap().ip().to_string(), err.unwrap()));
        write_invalid_entry_response(&stream, String::from("Failed to read entry point: invalid!"));
        disconnect_client(username);
        return;
    }
    if !login.is_some() { // actual login attempt
        disconnect_client(username);
        return;
    }

    // Send login response and process data
    let l: LoginData = login.unwrap();
    global_logger.lock().unwrap().info(format!("Login data:\n > username: {}\n > passwd: {}\n > signup: {}\n > signup key: {}",
             l.username, l.passwd, l.signup, l.signup_key));
    // TODO: validate login attempt & store username for use (password not stored in memory)
    let username = l.username;
    write_valid_entry_response(&stream, motd); // TODO: remove. for now, just send a valid entry response to ensure that testing works

    let mut last_keepalive = SystemTime::now();
    let mut expecting_keepalive = false;
    let mut ping = 0;

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
                disconnect_client(username);
                return;
            }
        }

        // listen for event packet
        // TODO: Fix this issue
        //  This can continue on listening until the server closes because it waits for the event packet (which could be a keepalive) until it receives something,
        //  even if the connection is dead.
        let (msg, raw, time, error, disconnect_status) = read_event(&stream);
        disconnect = disconnect_status;
        if msg.is_some() {
            // The client has sent a message and should be sent to other clients
            let md = msg.unwrap();
            let msg_str = md.msg;
            let msg_color = md.color;
            let name = md.name;
            let name_color = md.name_color;

            // TODO: send message to the server for other clients to see

        } else if raw.is_some() {
            // Clients do not have permission to send raw messages, return an error event
            write_event_error(&stream, String::from("An invalid raw event was sent to the server: raw events cannot originate from a client!"));
            // no need to disconnect, not a fatal error
        } else if time.is_some() {
            if !expecting_keepalive {
                // Not expecting a keepalive, ignore
                continue;
            }
            let client_time = time.unwrap();
            // calculate ping

        } else {
            if error.is_some() { // if the client sent an error message, log it in the server (maybe implement error handling later?)
                let err = error.unwrap();
                global_logger.lock().unwrap().error(format!("Client {} has sent an error event: {}", username, err));
            } else { // if the client didnt send an error, then an invalid packet was sent.
                global_logger.lock().unwrap().error(format!("Client {} has sent an invalid packet or a fatal error has occured with the connection. They have been disconnected.", username));
            }

            disconnect = true;
        }

        if disconnect { // if the disconnect flag is set, disconnect the client
            disconnect_client(username);
            drop(stream); // not needed, but for clarity
            return;
        }

    }

}