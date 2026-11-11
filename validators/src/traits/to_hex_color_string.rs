use alloc::string::String;

/// The `hex_color` validator will implement this for its types.
pub trait ToHexColorString {
    /// Retrieve the color as a hex string.
    fn to_hex_color_string(&self) -> String;
}
