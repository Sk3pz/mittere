use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use mittere_lib::network::entry_point_io::read_entry_point;
use mittere_lib::logger::Logger;
use mittere_lib::network::login_data::LoginData;

pub fn handle_client(stream: TcpStream, mut logger: Logger, sender: Sender<String>) {
    let (login, version, err) = read_entry_point(&stream);
    if login.is_some() {
        let l: LoginData = login.unwrap();
        logger.info(format!("Login data:\n > username: {}\n > passwd: {}\n > signup: {}\n > signup key: {}",
        l.username, l.passwd, l.signup, l.signup_key));
    }
    if version.is_some() {
        logger.info(format!("Client is on version {}", version.unwrap()));
    }
    if err.is_some() {
        logger.error(format!("Encountered an error reading entry point for client {}:\n{}", stream.peer_addr().unwrap().ip().to_string(), err.unwrap()));
    }
}