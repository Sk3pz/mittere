use std::net::{TcpListener, TcpStream};

use send_it::{reader::VarReader, writer::VarWriter};

#[derive(Debug, Clone)]
pub struct ConnectionInfo {
    pub(crate) ip: String,
    pub(crate) port: u16,
}

fn main() {
    println!("Hello, world!");

    let listener = match TcpStream::connect("127.0.0.1:2727") {
        Ok(l) => l,
        Err(e) => {
            println!("Error connecting to {}: {}", "", e);
            std::process::exit(1);
        }
    };
    let t = tokio::spawn(async move { loop {} });
}
