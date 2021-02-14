// Automatically generated rust module for 'pbuff.proto' file

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unknown_lints)]
#![allow(clippy::all)]
#![cfg_attr(rustfmt, rustfmt_skip)]


use std::borrow::Cow;
use quick_protobuf::{MessageRead, MessageWrite, BytesReader, Writer, WriterBackend, Result};
use quick_protobuf::sizeofs::*;
use super::*;

#[derive(Debug, Default, PartialEq, Clone)]
pub struct ConnectionError<'a> {
    pub err: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for ConnectionError<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.err = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for ConnectionError<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.err == "" { 0 } else { 1 + sizeof_len((&self.err).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.err != "" { w.write_with_tag(10, |w| w.write_string(&**&self.err))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Login<'a> {
    pub username: Cow<'a, str>,
    pub password: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Login<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.username = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.password = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Login<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.username == "" { 0 } else { 1 + sizeof_len((&self.username).len()) }
        + if self.password == "" { 0 } else { 1 + sizeof_len((&self.password).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.username != "" { w.write_with_tag(10, |w| w.write_string(&**&self.username))?; }
        if self.password != "" { w.write_with_tag(18, |w| w.write_string(&**&self.password))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct LoginValidate {
    pub validated: bool,
}

impl<'a> MessageRead<'a> for LoginValidate {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.validated = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for LoginValidate {
    fn get_size(&self) -> usize {
        0
        + if self.validated == false { 0 } else { 1 + sizeof_varint(*(&self.validated) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.validated != false { w.write_with_tag(8, |w| w.write_bool(*&self.validated))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct JoinChannel<'a> {
    pub channel_list: Cow<'a, str>,
    pub channel_msg_history: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for JoinChannel<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.channel_list = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.channel_msg_history = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for JoinChannel<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.channel_list == "" { 0 } else { 1 + sizeof_len((&self.channel_list).len()) }
        + if self.channel_msg_history == "" { 0 } else { 1 + sizeof_len((&self.channel_msg_history).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.channel_list != "" { w.write_with_tag(10, |w| w.write_string(&**&self.channel_list))?; }
        if self.channel_msg_history != "" { w.write_with_tag(18, |w| w.write_string(&**&self.channel_msg_history))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SignupKey<'a> {
    pub key: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for SignupKey<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.key = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for SignupKey<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.key == "" { 0 } else { 1 + sizeof_len((&self.key).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.key != "" { w.write_with_tag(10, |w| w.write_string(&**&self.key))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct SignupKeyValidate {
    pub validated: bool,
}

impl<'a> MessageRead<'a> for SignupKeyValidate {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.validated = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for SignupKeyValidate {
    fn get_size(&self) -> usize {
        0
        + if self.validated == false { 0 } else { 1 + sizeof_varint(*(&self.validated) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.validated != false { w.write_with_tag(8, |w| w.write_bool(*&self.validated))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Signup<'a> {
    pub username: Cow<'a, str>,
    pub password: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Signup<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.username = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.password = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Signup<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.username == "" { 0 } else { 1 + sizeof_len((&self.username).len()) }
        + if self.password == "" { 0 } else { 1 + sizeof_len((&self.password).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.username != "" { w.write_with_tag(10, |w| w.write_string(&**&self.username))?; }
        if self.password != "" { w.write_with_tag(18, |w| w.write_string(&**&self.password))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Disconnect {
    pub error: bool,
}

impl<'a> MessageRead<'a> for Disconnect {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(8) => msg.error = r.read_bool(bytes)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl MessageWrite for Disconnect {
    fn get_size(&self) -> usize {
        0
        + if self.error == false { 0 } else { 1 + sizeof_varint(*(&self.error) as u64) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.error != false { w.write_with_tag(8, |w| w.write_bool(*&self.error))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct Msg<'a> {
    pub display_name: Cow<'a, str>,
    pub name_color: Cow<'a, str>,
    pub msg: Cow<'a, str>,
    pub msg_color: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for Msg<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.display_name = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(18) => msg.name_color = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(26) => msg.msg = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(34) => msg.msg_color = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for Msg<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.display_name == "" { 0 } else { 1 + sizeof_len((&self.display_name).len()) }
        + if self.name_color == "" { 0 } else { 1 + sizeof_len((&self.name_color).len()) }
        + if self.msg == "" { 0 } else { 1 + sizeof_len((&self.msg).len()) }
        + if self.msg_color == "" { 0 } else { 1 + sizeof_len((&self.msg_color).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.display_name != "" { w.write_with_tag(10, |w| w.write_string(&**&self.display_name))?; }
        if self.name_color != "" { w.write_with_tag(18, |w| w.write_string(&**&self.name_color))?; }
        if self.msg != "" { w.write_with_tag(26, |w| w.write_string(&**&self.msg))?; }
        if self.msg_color != "" { w.write_with_tag(34, |w| w.write_string(&**&self.msg_color))?; }
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
pub struct MOTD<'a> {
    pub motd: Cow<'a, str>,
}

impl<'a> MessageRead<'a> for MOTD<'a> {
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self> {
        let mut msg = Self::default();
        while !r.is_eof() {
            match r.next_tag(bytes) {
                Ok(10) => msg.motd = r.read_string(bytes).map(Cow::Borrowed)?,
                Ok(t) => { r.read_unknown(bytes, t)?; }
                Err(e) => return Err(e),
            }
        }
        Ok(msg)
    }
}

impl<'a> MessageWrite for MOTD<'a> {
    fn get_size(&self) -> usize {
        0
        + if self.motd == "" { 0 } else { 1 + sizeof_len((&self.motd).len()) }
    }

    fn write_message<W: WriterBackend>(&self, w: &mut Writer<W>) -> Result<()> {
        if self.motd != "" { w.write_with_tag(10, |w| w.write_string(&**&self.motd))?; }
        Ok(())
    }
}

