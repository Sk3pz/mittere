use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use mittere_lib::network::entry_point_io::read_entry_point;
use mittere_lib::logger::Logger;
use mittere_lib::network::login_data::LoginData;
use crate::SERVER_VERSION;
use mittere_lib::network::entry_response_io::{write_ping_entry_response, write_valid_entry_response, write_invalid_entry_response};
use mittere_lib::network::event_io::read_event;

// returns true if disconnected
pub fn check_disconnected(stream: &TcpStream) -> bool {
    let mut dummy_buffer: &mut [u8; 8] = &mut [0; 8];
    stream.peek(dummy_buffer).is_err()
}

pub fn handle_client(stream: TcpStream, sender: Sender<String>, motd: String) {
    // handle EntryPoint
    let (login, version, err) = read_entry_point(&stream);
    if version.is_some() { // just a 'ping' to check the server is compatible
        let server_version = String::from(SERVER_VERSION);
        write_ping_entry_response(&stream, server_version == version.unwrap(), server_version);
        return;
    }
    if err.is_some() { // something went wrong
        println!("Encountered an error reading entry point for client {}:\n{}", stream.peer_addr().unwrap().ip().to_string(), err.unwrap());
        write_invalid_entry_response(&stream, String::from("Failed to read entry point: invalid!"));
        return;
    }
    if !login.is_some() { // actual login attempt
        return;
    }

    // Send login response and process data
    let l: LoginData = login.unwrap();
    println!("Login data:\n > username: {}\n > passwd: {}\n > signup: {}\n > signup key: {}",
             l.username, l.passwd, l.signup, l.signup_key);
    write_valid_entry_response(&stream, motd); // TODO: Check valid login

    // Wait for scheduled config update packet

    // main client connection loop: Process Event packets
    loop {
        // ensure the connection is still open
        if check_disconnected(&stream) {
            return; // if disconnected, return
        }

        // listen for event packet
        let (msg, error) = read_event(&stream);
        if msg.is_some() {
            // the message should be processed
            // TODO: send message to the server for other clients to see
            let m = msg.unwrap();
            let msg_str = m.msg;
            let msg_color = m.color;
            let name = m.name;
            let name_color = m.name_color;
        } else if error.is_some() {
            
        }

    }

}