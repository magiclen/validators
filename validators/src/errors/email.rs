use core::fmt::{self, Display, Formatter};

#[cfg(feature = "std")]
use std::error::Error;

#[derive(Debug, Clone)]
pub enum EmailError {
    /// the fallback variant
    Invalid,
    /// may not be valid but it is guaranteed that the domain part is not an IP
    IPMust,
    /// may not be valid and the domain part seems to be an IP
    IPNotAllow,
    /// may not be valid but it is guaranteed that the domain part is not local
    LocalMust,
    /// may not be valid but it is guaranteed that the domain part is local
    LocalNotAllow,
    /// may not be valid but it is guaranteed that the domain part has only one label
    AtLeastTwoLabelsMust,
    /// may not be valid and the domain part seems to has at least two labels
    AtLeastTwoLabelsNotAllow,
    /// may not be valid and comments seems to exist
    CommentNotAllow,
}

impl Display for EmailError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            EmailError::Invalid => f.write_str("invalid Email"),
            EmailError::IPMust => f.write_str("must use an IP"),
            EmailError::IPNotAllow => f.write_str("must not use an IP"),
            EmailError::LocalMust => f.write_str("must be local"),
            EmailError::LocalNotAllow => f.write_str("must not be local"),
            EmailError::AtLeastTwoLabelsMust => f.write_str("must have at least two labels"),
            EmailError::AtLeastTwoLabelsNotAllow => f.write_str("must have only one label"),
            EmailError::CommentNotAllow => f.write_str("must not contain comments"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for EmailError {}
