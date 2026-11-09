use core::fmt::{self, Display, Formatter};

/// Error from the `hex_decoded` validator.
#[derive(Debug, Clone)]
pub struct HexDecodedError;

impl Display for HexDecodedError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("invalid hexadecimal data")
    }
}

impl core::error::Error for HexDecodedError {}
