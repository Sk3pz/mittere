// Mittere client

use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::io::{Read, Write};
use better_term::style::{Style, Color};
use mittere_lib::make_logger;
use mittere_lib::network::entry_point_io::{write_entry_point_ver, write_entry_point_signup};
use mittere_lib::network::entry_response_io::read_entry_response;

fn connection_err() {
    eprintln!("{}Failed to connect to the Mittere server: The connection is not available or was refused. \
        Maybe check the IP and Port?\n > IP: {}\n > PORT: {}", Color::Red, ip, port);
}

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
    let mut stream_result = TcpStream::connect(address.clone());
    if stream_result.is_err() {
        connection_err();
        return;
    }
    let mut stream = stream_result.expect("Uh oh! I made an oopsie! Please contact the developer and explain you got an error code 01C");
    write_entry_point_ver(&stream, version); // send the "ping" packet
    let (valid, _, server_version, err) = read_entry_response(&stream);
    if server_version.is_some() {
        println!("Valid: {}\nserver version: {}", valid, server_version.unwrap());
    } else {
        if err.is_some() {
            println!("Valid: {}\nerror: {}", valid, err.unwrap());
        } else {
            println!("There was an issue reading the ping response. Closing connection");
        }
        return;
    }
    drop(stream); // stop the ping connection

    // TODO: Login stuff here
    let username = String::from("SomeRandomUsername");
    let password = String::from("SomeRandomPassword");
    let signup_key = String::from("SomeRandomKey");
    let signup = true;

    // establish new connection
    stream_result = TcpStream::connect(address);
    if stream_result.is_err() {
        connection_err();
        return;
    }
    stream = stream_result.unwrap();

    // send a signup / login request
    write_entry_point_signup(&stream, username, password, signup, signup_key);

    // wait for response and store output
    let (login_valid, login_motd, _, login_err) = read_entry_response(&stream);

    if !login_valid {
        println!("The login or signup was not valid.");
        if login_err.is_some() {
            println!("Error: {}", login_err.unwrap());
        }
        return;
    }
    if login_motd.is_some() {
        println!("Connected to the server! MOTD:\n{}", login_motd.unwrap());
    }

    drop((tx, rx));
}
