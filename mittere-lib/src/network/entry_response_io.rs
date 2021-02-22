use capnp::message::{TypedReader, Builder, HeapAllocator};
use crate::packet_capnp::entry_response;
use std::borrow::Borrow;
use capnp::serialize;
use std::net::TcpStream;

pub fn write_valid_entry_response(mut stream: &TcpStream, motd: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut er = message.init_root::<entry_response::Builder>();
        er.set_valid(true);
        er.set_motd(motd.as_str());
    }
    serialize::write_message(&mut stream, &message)
}

pub fn write_invalid_entry_response(mut stream: &TcpStream, err: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut er = message.init_root::<entry_response::Builder>();
        er.set_valid(false);
        er.set_error(err.as_str());
    }
    serialize::write_message(&mut stream, &message)
}

pub fn write_ping_entry_response(mut stream: &TcpStream, client_valid: bool, version: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut er = message.init_root::<entry_response::Builder>();
        er.set_valid(client_valid);
        er.set_version(version.as_str());
    }
    serialize::write_message(&mut stream, &message)
}

/// returns valid, motd, version, error
pub fn read_entry_response(mut stream: &TcpStream) -> (bool, Option<String>, Option<String>, Option<String>) {
    let message_reader_result = serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new());
    if message_reader_result.is_err() {
        return (false, None, None, Some(String::from("Could not connect to server.")));
    }
    let message_reader = message_reader_result.unwrap();
    let er = message_reader.get_root::<entry_response::Reader>().expect("Uh oh 2!");

    return match er.which() {
        Ok(entry_response::Version(v)) => {
            (er.get_valid(), None, Some(v.unwrap().to_string()), None)
        }
        Ok(entry_response::Motd(motd)) => {
            (er.get_valid(), Some(motd.unwrap().to_string()), None, None)
        }
        Ok(entry_response::Error(err)) => {
            (er.get_valid(), None, None, Some(err.unwrap().to_string()))
        }
        Err(::capnp::NotInSchema(_)) => {
            (false, None, None, Some(String::from("Invalid EntryResponse - no data found!")))
        }
    }
}