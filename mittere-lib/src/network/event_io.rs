use std::net::TcpStream;
use capnp::message::{TypedReader, Builder, HeapAllocator};
use crate::packet_capnp::event;
use capnp::serialize;
use better_term::style::Color;
use crate::network::msg_data::MessageData;

pub fn write_event_message(mut stream: &TcpStream, msg_str: String, display_name: String, name_color: Color, msg_color: Color) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut ev = message.init_root::<event::Builder>();
        let mut msg = ev.init_message();
        {
            msg.set_msg(msg_str.as_str());
            msg.set_display_name(display_name.as_str());
            msg.set_msg_color(msg_color.as_fg().as_str());
            msg.set_name_color(name_color.as_fg().as_str());
        }
    }
    serialize::write_message(&mut stream, &message)
}

pub fn write_event_error(mut stream: &TcpStream, error: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut ev = message.init_root::<event::Builder>();
        ev.set_error(error.as_str());
    }
    serialize::write_message(&mut stream, &message)
}

// Returns Message data or error
pub fn read_event(mut stream: &TcpStream) -> (Option<MessageData>, Option<String>) {
    let message_reader = serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new()).expect("Uh oh!");
    let ep = message_reader.get_root::<event::Reader>().expect("Uh oh 2!");

    return match ep.which() {
        Ok(event::Message(msg)) => {
            let raw_md = msg.unwrap();
            let md = MessageData {
                msg: raw_md.get_msg().unwrap().to_string(),
                color: raw_md.get_msg_color().unwrap().to_string(),
                name: raw_md.get_display_name().unwrap().to_string(),
                name_color: raw_md.get_name_color().unwrap().to_string()
            };
            (Some(md), None)
        }
        Ok(event::Error(err)) => {
            (None, Some(err.unwrap().to_string()))
        }
        Err(::capnp::NotInSchema(_)) => {
            // todo: error
            (None, Some(String::from("Invalid EntryPoint - no version or login data found!")))
        }
    }
}