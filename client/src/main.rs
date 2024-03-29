use tokio::net::TcpStream;
use better_term::read_input;

use send_it::{async_reader::VarReader, async_writer::VarWriter};
use common::message::Message;
use common::to_local_time;

#[tokio::main]
async fn main() {
    let ip = read_input!("Enter the server IP: ");
    let port = read_input!("Enter the server port: ");

    let stream = match TcpStream::connect(format!("{}:{}", ip, port).as_str()).await {
        Ok(l) => l,
        Err(e) => {
            println!("Error connecting to {}: {}", "", e);
            std::process::exit(1);
        }
    };

    // try to clone the stream
    let (mut stream_read, mut stream_write) = stream.into_split();

    tokio::spawn(async move {
        let mut reader = VarReader::new(&mut stream_read);
        loop {
            while let Ok(read) = reader.read_data().await {
                 let message = match Message::from_segments(read) {
                     Ok(m) => m,
                     Err(e) => {
                         eprintln!("Error reading message: {}", e);
                         continue;
                     }
                 };

                let local_time = match to_local_time(&message.timestamp) {
                    Ok(t) => t,
                    Err(e) => {
                        eprintln!("Error parsing time: {}", e);
                        continue;
                    }
                };

                println!("{} {}: {}", local_time.format("%m/%d/%Y %I:%M%p"), message.author, message.message);
            }

            // exit the program
            println!("Disconnected from server :(");
            std::process::exit(0);
        }
    });

    // get username
    println!("Connected to server! :D");
    let username = read_input!("Enter your username: ");
    let mut writer = VarWriter::new();
    writer.add_string(username);
    writer.send(&mut stream_write).await.expect("Failed to send :(");

    // stdin message
    let mut writer = VarWriter::new();
    loop {
        let input = read_input!();
        writer.add_string(input);
        writer.send(&mut stream_write).await.unwrap_or_else(|e| {
            eprintln!("Failed to send: {}", e);
        });
    }
}
