use alloc::string::String;

/// The `json` validator will implement this for its types.
pub trait ToJsonString {
    /// Retrieve this json element as a minified string.
    ///
    /// # Panics
    ///
    /// Panics if the inner value cannot be written as JSON, such as a map whose keys are not strings.
    fn to_minified_json_string(&self) -> String;

    /// Retrieve this json element as a beautified string.
    ///
    /// # Panics
    ///
    /// Panics if the inner value cannot be written as JSON, such as a map whose keys are not strings.
    fn to_beautified_json_string(&self) -> String;
}
