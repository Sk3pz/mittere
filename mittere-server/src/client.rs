use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};

pub fn handle_client(stream: TcpStream, sender: Sender<String>) {

}