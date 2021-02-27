// Mittere client

mod input;

use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::io::{Read, Write};
use better_term::style::{Style, Color};
use mittere_lib::make_logger;
use mittere_lib::network::entry_point_io::{write_entry_point_ver, write_entry_point_signup};
use mittere_lib::network::entry_response_io::read_entry_response;
use std::thread;
use crate::input::input_handler;
use mittere_lib::network::event_io::{read_event, write_event_error, write_event_keepalive};
use std::path::Path;

fn connection_err(ip: &str, port: &str) {
    eprintln!("{}Failed to connect to the Mittere server: The connection is not available or was refused. \
        Maybe check the IP and Port?\n > IP: {}\n > PORT: {}", Color::Red, ip, port);
}

fn main() {

    // get the client version from the environment
    let version = String::from(env!("CARGO_PKG_VERSION"));

    // create channel for communicating between the input thread and the main loop
    let (tx, rx): (Sender<String>, Receiver<String>) = channel();

    // get config values
    // TODO: make these values be configurable or asked for in the login process
    let ip = "localhost";
    let port = "2277";
    let address = format!("{}:{}", ip, port);

    // Establish a temporary connection to check if the server is up and if it is on a compatible version
    let mut stream_result = TcpStream::connect(address.clone());
    if stream_result.is_err() {
        connection_err(ip, port);
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
            connection_err(ip, port);
        }
        return;
    }
    drop(stream); // stop the ping connection

    // TODO: Login stuff here
    let username = String::from("Hroc");
    let password = String::from("SomeRandomPassword");
    let signup_key = String::from("SomeRandomKey");
    let signup = true;

    // establish new connection
    stream_result = TcpStream::connect(address);
    if stream_result.is_err() {
        connection_err(ip, port);
        return;
    }
    stream = stream_result.unwrap();

    // send a signup / login request
    write_entry_point_signup(&stream, username, password, signup, signup_key);

    // wait for response and store output
    let (login_valid, login_motd, _, login_err) = read_entry_response(&stream);

    if !login_valid || login_motd.is_none() {
        println!("The login or signup was not valid.");
        if login_err.is_some() {
            println!("{}[{}ERROR{}] {}> {}The server has sent an error: {}", Color::BrightBlack, Color::BrightRed, Color::BrightBlack, Color::Red, Color::BrightRed, login_err.unwrap());
        }
        return;
    }
    let motd = login_motd.unwrap();
    println!("{}", motd); // print the MOTD

    // Handle chats input and output

    // handle input thread
    let stream_clone = stream.try_clone();
    thread::spawn(|| input_handler(stream_clone.expect("Failed to create copy of TcpStream for input handler. If this issue persists, please contact the developer.")));

    loop {
        let (msg, cfg, time, error, disconnect_status) = read_event(&stream);
        if msg.is_some() {
            let m = msg.unwrap();
            println!("{}", m);
            println!("{}", Color::White);
        } else if cfg.is_some() {
            write_event_error(&stream, String::from("Invalid packet from the server: Not expecting a config update event."));
            println!("{}[{}WARNING{}] {}> {}An unexpected packet was received from the server.", Color::BrightBlack, Color::BrightYellow, Color::BrightBlack, Color::White, Color::BrightYellow);
        } else if time.is_some() {
            write_event_keepalive(&stream);
        } else {
            if error.is_some() {
                println!("{}[{}ERROR{}] {}> {}The server has sent an error: {}", Color::BrightBlack, Color::BrightRed, Color::BrightBlack, Color::Red, Color::BrightRed, error.unwrap());
            } else {
                println!("{}[{}WARNING{}] {}> {}An unexpected packet was received from the server.", Color::BrightBlack, Color::BrightYellow, Color::BrightBlack, Color::White, Color::BrightYellow);
            }
        }
        if disconnect_status {
            println!("{}[{}Mittere{}] {}> {}The server has disconnected you.", Color::BrightBlack, Color::Cyan, Color::BrightBlack, Color::White, Color::BrightCyan);
        }
    }

    drop((tx, rx));
}
