use std::net::TcpStream;
use capnp::message::{TypedReader, Builder, HeapAllocator};
use crate::packet_capnp::event;
use capnp::serialize;
use better_term::style::Color;
use crate::network::cfg_data::ConfigData;
use crate::systime;

/// Sends a message event
pub fn write_event_message(mut stream: &TcpStream, msg_str: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut ev = message.init_root::<event::Builder>();
        ev.set_message(msg_str.as_str());
    }
    serialize::write_message(&mut stream, &message)
}

/// Sends a message event to the server
pub fn write_cfg_update(mut stream: &TcpStream, display_name: String, display_color: String, msg_color: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut ev = message.init_root::<event::Builder>();
        let mut cfg = ev.init_config();
        {
            cfg.set_display_name(display_color.as_str());
            cfg.set_msg_color(msg_color.as_str());
            cfg.set_name_color(display_color.as_str());
        }
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
/// Returns message, config data, keepalive_time, an error, and a disconnect flag
pub fn read_event(mut stream: &TcpStream) -> (Option<String>, Option<ConfigData>, Option<u64>, Option<String>, bool) {
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
            (Some(msg.unwrap().to_string()), None, None, None, needs_to_disconnect)
        }
        Ok(event::Config(cfg)) => {
            let raw_cfg = cfg.unwrap();
            let cfg = ConfigData {
                display_name: raw_cfg.get_display_name().unwrap().to_string(),
                name_color: raw_cfg.get_name_color().unwrap().to_string(),
                msg_color: raw_cfg.get_msg_color().unwrap().to_string()
            };
            (None, Some(cfg), None, None, needs_to_disconnect)
        }
        Ok(event::Keepalive(st)) => {
            (None, None, Some(st), None, ev.get_disconnect())
        }
        Ok(event::Error(err)) => {
            (None, None, None, Some(err.unwrap().to_string()), needs_to_disconnect)
        }
        Err(::capnp::NotInSchema(_)) => {
            // todo: error
            (None, None, None, Some(String::from("Invalid EntryPoint - no version or login data found!")), needs_to_disconnect)
        }
    }
}