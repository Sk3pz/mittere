pub mod entry_point_io;
pub mod entry_response_io;
pub mod login_data;

use std::net::TcpStream;
use std::io::{Error, Read};

/// attempt to read data from a stream
///
/// # Arguments
///
/// * `stream` - A TcpStream to read the data from
/// * `size` - A usize that sets how many bytes should be read from the stream.
///
/// # Return
///
/// returns a vector of bytes if successful, an std::io::Error if not
///
pub fn read_bytes(mut stream: TcpStream, size: usize) -> Result<Vec<u8>, Error> {
    // the buffer to read into
    let mut buffer = vec![0; size];
    // attempt to read the data from the stream into the buffer with a possible failure
    let result = stream.read_exact(&mut buffer);

    // if an error occurred with reading, return the error
    if let Err(err) = result {
        return Err(err); // TODO: disconnect the client with the error???
    }

    Ok(buffer)
}