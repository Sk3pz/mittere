use std::net::TcpStream;
use crate::network::packet::PacketType;
use crate::network::PACKET_VAR_SEPARATOR;

/// sends a packet containing data over a stream
///
/// # Arguments
///
/// * `stream` - A TcpStream to attempt to send the packet over
/// * `ptype` - A PacketType that tells the receiver what type of data to parse
/// * `data` - A Vec of Strings that holds the different sets of data (see the packet type's layout for better explanation)
///
/// # Return
///
/// returns true if successful
///
pub fn send_data_packet(stream: &mut TcpStream, ptype: PacketType, data: Vec<String>) -> bool {
    // create a string from the data using the join character
    let joined_data = ptype.get_type_id().to_string() + PACKET_VAR_SEPARATOR + data.join(PACKET_VAR_SEPARATOR);
    let data_size = joined_data.len();

    // first send the packet leader which tells the reader the correct size of the incoming packet
}

/// sends a packet containing no data over a stream
///
/// # Arguments
///
/// * `stream` - A TcpStream to attempt to send the packet over
/// * `ptype` - A PacketType that tells the receiver what type of data to parse
///
/// # Return
///
/// returns true if successful
///
pub fn send_non_data_packet(stream: &mut TcpStream, ptype: PacketType) -> bool {

}