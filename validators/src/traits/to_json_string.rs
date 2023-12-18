use alloc::string::String;

/// The `json` validator will implement this for its types.
pub trait ToJsonString {
    /// Retrieve this json element as a minified string.
    fn to_minified_json_string(&self) -> String;

    /// Retrieve this json element as a beautified string.
    fn to_beautified_json_string(&self) -> String;
}
