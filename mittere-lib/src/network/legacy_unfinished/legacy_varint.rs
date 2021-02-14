// An unfinished implementation of a VarInt for use in networking - switched to using Protocol Buffers so there is no longer any need

/// A VarInt is an integer that uses the MSB (Most Significant Bit) to indicate if there are
/// further bytes to come.

/// Endianness:
/// Big stores the 'big-end' first (MSB -> LSB)
/// Little stores the 'little-end' first (LSB -> MSB)

/// How it works:
/// set the MSB in each byte of the number to on if there are more bytes ahead

/// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
///
/// WARNING: THE PUREST FORM OF RAW AGONY AND PAIN AHEAD!!!
///
/// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

pub struct VarInt(Vec<u8>);

impl VarInt {
    pub fn new() -> Self {
        VarInt {
            0: vec![],
        }
    }

    pub fn from_existing(bytes: Vec<u8>) -> Self {
        VarInt {
            0: bytes
        }
    }

    /// Convert a value into a byte vec using 7 bytes to store and the MSB as a flag to indicate more bytes
    pub fn from(mut value: usize) -> VarInt {
        let mut vi: Vec<u8> = vec![];
        while {
            let mut temp = (value & 0b01111111) as u8;
            value >>= 7;
            if value != 0 {
                temp |= 0b10000000;
            }
            vi.push(temp);
            value != 0
        } {}
        VarInt::from_existing(vi)
    }

    /// Converts a byte vec in VarInt form into a usize
    pub fn to(self) -> usize {
        unimplemented!()
    }
}

impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.to()
    }
}
