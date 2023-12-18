use alloc::string::String;

/// The `uuid` validator will implement this for its types.
pub trait ToUuidString {
    /// Retrieve the UUID as a string.
    fn to_uuid_string(&self) -> String;
}
