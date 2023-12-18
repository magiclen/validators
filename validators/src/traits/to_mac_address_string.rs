use alloc::string::String;

/// The `mac_address` validator will implement this for its types.
pub trait ToMacAddressString {
    /// Retrieve the mac address as a string.
    fn to_mac_address_string(&self) -> String;
}
