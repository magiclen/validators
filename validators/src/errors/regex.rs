use core::fmt::{self, Display, Formatter};

/// Error from the `regex` validator.
#[derive(Debug, Clone)]
pub struct RegexError;

impl Display for RegexError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        f.write_str("invalid format")
    }
}

impl core::error::Error for RegexError {}
