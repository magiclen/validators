use alloc::borrow::Cow;

/// The `domain`, `host`, `ip`, `ipv4`, `ipv6` validators will implement this for their types.
pub trait ToUriAuthorityString {
    /// Retrieve the URI authority as a string.
    fn to_uri_authority_string(&self) -> Cow<'_, str>;
}
