/// A var int is an integer that uses the MSB (Most Significant Bit) to indicate if there are
/// further bytes to come.
pub struct VarInt(usize);

/// Little: 0000 0101
/// Big:    1010 0000

/// How it works:
/// set the MSB in each byte of the number to on if there are more bytes ahead

/// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
/// WARNING: THE PUREST FORM OF RAW AGONY AND PAIN AHEAD!!!
/// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
impl VarInt {
    /// "unwrap" the number into bytes to be sent
    pub fn unwrap(&self) -> Vec<u8> {
        // can't just convert number to bytes because that would be too easy

    }

    /// "wrap" the bytes back into a number using bit shifting
    pub fn wrap(bytes: Vec<u8>) -> VarInt {
        unimplemented!()
    }
}

/// Not sure this is needed, but ¯\_( ͡° ͜ʖ ͡°)_/¯
impl Into<usize> for VarInt {
    fn into(self) -> usize {
        self.0
    }
}


