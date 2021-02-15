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