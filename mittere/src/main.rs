// Mittere client

use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::io::{Read, Write};
use better_term::style::{Style, Color};
use mittere_lib::make_logger;
use mittere_lib::network::entry_point_io::{write_entry_point_ver, write_entry_point_signup};
use mittere_lib::network::entry_response_io::read_entry_response;

/*
    TODO: Reading system:
            1.) Read 6 bytes:
                - First 2 bytes: packet type
                - Last 4 bytes: data size (max size of 1024 or 9999 depending on how I feel)
            2.) Then read the rest of the data based on the size
            3.) send data to where it needs to go
*/

fn main() {

    let version = String::from(env!("CARGO_PKG_VERSION"));

    // create channel for communicating between the input thread and the main loop
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    // get config values
    // TODO: make these values be configurable or asked for in the login process
    let ip = "localhost";
    let port = "8080";
    let address = format!("{}:{}", ip, port);

    // Establish a temporary connection to check if the server is up and if it is on a compatible version
    let mut stream_result = TcpStream::connect(address);
    if stream_result.is_err() {
        eprintln!("{}Failed to connect to the Mittere server: The connection is not available or was refused. \
        Maybe check the IP and Port?\n > IP: {}\n > PORT: {}", Color::Red, ip, port);
        return;
    }
    let mut stream = stream_result.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 01C");
    write_entry_point_ver(&stream, version); // send the "ping" packet
    let (valid, motd, server_version, err) = read_entry_response(&stream);
    if motd.is_some() {
        println!("Valid: {}\nMOTD:\n{}", valid, motd.unwrap());
    }
    if server_version.is_some() {
        println!("Valid: {}\nserver version: {}", valid, server_version.unwrap());
    }
    if err.is_some() {
        println!("Valid: {}\nerror: {}", valid, err.unwrap());
    }
    drop(stream); // stop the ping connection

    drop((tx, rx));
    
}
