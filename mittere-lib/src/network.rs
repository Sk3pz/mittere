pub mod packet;
pub mod send;
pub mod receive;
pub mod varint;

/// The separator between data values in a packet
// using newline character because nobody can use that and no packet will send newlines >:)
const PACKET_VAR_SEPARATOR: &str = "\n";

/// Sets the maximum length of packets in bytes
/// TODO
const PACKET_LENGTH_LIMIT: usize = 16;