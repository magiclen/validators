use core::fmt::{self, Display, Formatter};

/// Error from the `hex_color` validator.
#[derive(Debug, Clone)]
pub struct HexColorError;

impl Display for HexColorError {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("invalid hexadecimal color")
    }
}

impl core::error::Error for HexColorError {}
