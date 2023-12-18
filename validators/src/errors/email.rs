use core::fmt::{self, Display, Formatter};
#[cfg(feature = "std")]
use std::error::Error;

/// Error from the `email` validator.
#[derive(Debug, Clone)]
pub enum EmailError {
    /// Incorrect email data.
    Invalid,
    /// May not be valid, but it is guaranteed that the domain part is not an IP.
    IPMust,
    /// May not be valid and the domain part seems to be an IP.
    IPDisallow,
    /// May not be valid, but it is guaranteed that the domain part is not local.
    LocalMust,
    /// May not be valid, but it is guaranteed that the domain part is local.
    LocalDisallow,
    /// May not be valid, but it is guaranteed that the domain part has only one label.
    AtLeastTwoLabelsMust,
    /// May not be valid and the domain part seems to has at least two labels.
    AtLeastTwoLabelsDisallow,
    /// May not be valid and comments seems to exist.
    CommentDisallow,
}

impl Display for EmailError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        match self {
            Self::Invalid => f.write_str("invalid Email"),
            Self::IPMust => f.write_str("must use an IP"),
            Self::IPDisallow => f.write_str("must not use an IP"),
            Self::LocalMust => f.write_str("must be local"),
            Self::LocalDisallow => f.write_str("must not be local"),
            Self::AtLeastTwoLabelsMust => f.write_str("must have at least two labels"),
            Self::AtLeastTwoLabelsDisallow => f.write_str("must have only one label"),
            Self::CommentDisallow => f.write_str("must not contain comments"),
        }
    }
}

#[cfg(feature = "std")]
impl Error for EmailError {}
