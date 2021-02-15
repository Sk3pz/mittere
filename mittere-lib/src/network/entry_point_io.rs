use capnp::message::{TypedReader, Builder, HeapAllocator};
use crate::packet_capnp::{entry_point, login};
use std::borrow::Borrow;
use capnp::serialize;
use std::net::TcpStream;
use crate::network::login_data::LoginData;

pub fn write_entry_point_ver(mut stream: &TcpStream, version: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let mut ep = message.init_root::<entry_point::Builder>();
        ep.set_version(version.as_str());
    }
    serialize::write_message(&mut stream, &message)
}

pub fn write_entry_point_login(mut stream: TcpStream, username: String, passwd: String) -> ::capnp::Result<()> {
    write_entry_point_signup(&stream, username, passwd, false, String::new())
}

pub fn write_entry_point_signup(mut stream: &TcpStream, username: String, passwd: String, signup: bool, signup_key: String) -> ::capnp::Result<()> {
    let mut message = Builder::new_default();
    {
        let ep = message.init_root::<entry_point::Builder>();
        let mut login = ep.init_login();
        {
            login.set_username(username.as_str());
            login.set_password(passwd.as_str());
            login.set_signup(signup);
            login.set_signup_key(signup_key.as_str());
        }
    }
    serialize::write_message(&mut stream, &message)
}

pub fn read_entry_point(mut stream: &TcpStream) -> (Option<LoginData>, Option<String>, Option<String>) {
    let message_reader = serialize::read_message(&mut stream, ::capnp::message::ReaderOptions::new()).expect("Uh oh!");
    let ep = message_reader.get_root::<entry_point::Reader>().expect("Uh oh 2!");

    return match ep.which() {
        Ok(entry_point::Login(login_data)) => {
            let raw_ld = login_data.unwrap();
            let ld = LoginData {
                username: raw_ld.get_username().unwrap().to_string(),
                passwd: raw_ld.get_password().unwrap().to_string(),
                signup: raw_ld.get_signup(),
                signup_key: raw_ld.get_signup_key().unwrap().to_string()
            };
            (Some(ld), None, None)
        }
        Ok(entry_point::Version(ver)) => {
            (None, Some(ver.unwrap().to_string()), None)
        }
        Err(::capnp::NotInSchema(_)) => {
            // todo: error
            (None, None, Some(String::from("Invalid EntryPoint - no version or login data found!")))
        }
    }
}