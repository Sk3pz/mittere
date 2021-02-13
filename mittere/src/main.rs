// Mittere client

use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::io::{Read, Write};
use mittere_lib::network::PacketType;
use better_term::style::{Style, Color};
use mittere_lib::make_logger;

/*
    TODO: Reading system:
            1.) Read 6 bytes:
                - First 2 bytes: packet type
                - Last 4 bytes: data size (max size of 1024 or 9999 depending on how I feel)
            2.) Then read the rest of the data based on the size
            3.) send data to where it needs to go
*/

fn main() {
    // create channel for communicating between the input thread and the main loop
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    // get config values
    // TODO: make these values be configurable or asked for in the login process
    let ip = "localhost";
    let port = "8080";
    let address = format!("{}:{}", ip, port);

    // Establish connection to start communication
    let stream_result = TcpStream::connect(address);
    if stream_result.is_err() {
        eprintln!("{}Failed to connect to the Mittere server: The connection is not available or was refused. \
        Maybe check the IP and Port?\n > IP: {}\n > PORT: {}", Color::Red, ip, port);
        return;
    }
    let stream = stream_result.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 01C");

    // TODO: Signup / Login system here

    // TODO: handle connection

    drop((tx, rx));
    
}
