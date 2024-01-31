use std::net::TcpStream;
use better_term::read_input;
use chrono::DateTime;

use send_it::{reader::VarReader, writer::VarWriter};
use common::message::Message;

#[tokio::main]
async fn main() {
    let ip = read_input!("Enter the server IP: ");
    let port = read_input!("Enter the server port: ");

    let mut stream = match TcpStream::connect(format!("{}:{}", ip, port).as_str()) {
        Ok(l) => l,
        Err(e) => {
            println!("Error connecting to {}: {}", "", e);
            std::process::exit(1);
        }
    };

    // try to clone the stream
    let mut stream_reader = match stream.try_clone(){
        Ok(s) => s,
        Err(e) => {
            println!("Error cloning stream: {}", e);
            return;
        }
    };

    tokio::spawn(async move {
        let mut reader = VarReader::new(&mut stream_reader);
        loop {
            while let Ok(read) = reader.read_data() {
                 let message = match Message::from_segments(read) {
                     Ok(m) => m,
                     Err(e) => {
                         eprintln!("Error reading message: {}", e);
                         continue;
                     }
                 };

                println!("timestamp: {}", message.timestamp);

                let local_time = match DateTime::parse_from_str(message.timestamp.as_str(), "%Y-%m-%d %H:%M:%S") {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Error parsing time: {}", e);
                        continue;
                    }
                };

                println!("{} {}: {}", local_time.naive_local(), message.author, message.message);
            }

            // exit the program
            std::process::exit(0);
        }
    });

    // get username
    println!("Connected to server! :D");
    let username = read_input!("Enter your username: ");
    let mut writer = VarWriter::new();
    writer.add_string(username);
    writer.send(&mut stream).expect("Failed to send :(");

    // stdin message
    let mut writer = VarWriter::new();
    loop {
        let input = read_input!();
        writer.add_string(input);
        writer.send(&mut stream).unwrap_or_else(|e| {
            eprintln!("Failed to send: {}", e);
        });
    }
}
