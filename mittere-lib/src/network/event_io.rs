use std::net::TcpStream;
use capnp::message::{TypedReader, Builder, HeapAllocator};
use crate::packet_capnp::event;
use capnp::serialize;
use better_term::style::Color;
use crate::network::msg_data::MessageData;
use crate::systime;

/// Sends a message event to the server
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

/// Sends a raw message to clients (no username or color data, just the message)
pub fn write_event_raw_msg(mut stream: &TcpStream, raw_msg: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut ev = message.init_root::<event::Builder>();
        ev.set_raw(raw_msg.as_str());
    }
    serialize::write_message(&mut stream, &message)
}

/// Sends a keepalive request or response
pub fn write_event_keepalive(mut stream: &TcpStream) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut ev = message.init_root::<event::Builder>();
        ev.set_keepalive(systime().as_secs());
    }
    serialize::write_message(&mut stream, &message)
}

/// Sends an error
pub fn write_event_error(mut stream: &TcpStream, error: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut ev = message.init_root::<event::Builder>();
        ev.set_error(error.as_str());
    }
    serialize::write_message(&mut stream, &message)
}

/// Reads an event packet, and returns it's data
/// Returns Message data, raw, keepalive_time, an error, and a disconnect flag
pub fn read_event(mut stream: &TcpStream) -> (Option<MessageData>, Option<String>, Option<u64>, Option<String>, bool) {
    let mut needs_to_disconnect = false;

    // read the event
    let message_reader_result = serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new());
    if message_reader_result.is_err() { // disconnected
        return (None, None, None, None, true);
    }
    let message_reader = message_reader_result.unwrap();
    // store the event in a Reader to obtain data out of it
    let ev = message_reader.get_root::<event::Reader>().expect("Could not form event from message_reader.");

    needs_to_disconnect = ev.get_disconnect();

    // the event is a Cap'n Proto Union, so go through which type of event it is
    return match ev.which() {
        Ok(event::Message(msg)) => {
            let raw_md = msg.unwrap();
            let md = MessageData {
                msg: raw_md.get_msg().unwrap().to_string(),
                color: raw_md.get_msg_color().unwrap().to_string(),
                name: raw_md.get_display_name().unwrap().to_string(),
                name_color: raw_md.get_name_color().unwrap().to_string()
            };
            (Some(md), None, None, None, needs_to_disconnect)
        }
        Ok(event::Raw(raw)) => {
            (None, Some(raw.unwrap().to_string()), None, None, needs_to_disconnect)
        }
        Ok(event::Keepalive(st)) => {
            (None, None, Some(st), None, ev.get_disconnect())
        }
        Ok(event::Error(err)) => {
            (None, Some(err.unwrap().to_string()), None, None, needs_to_disconnect)
        }
        Err(::capnp::NotInSchema(_)) => {
            // todo: error
            (None, Some(String::from("Invalid EntryPoint - no version or login data found!")), None, None, needs_to_disconnect)
        }
    }
}