use alloc::string::String;

/// The `email` validator will implement this for its types.
pub trait ToEmailString {
    /// Retrieve the email as a string.
    fn to_email_string(&self) -> String;
}
