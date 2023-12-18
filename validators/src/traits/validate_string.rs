use alloc::string::String;

/// Validate and deserialize strings.
pub trait ValidateString: Sized {
    type Error;

    fn parse_string<S: Into<String>>(s: S) -> Result<Self, Self::Error>;

    fn parse_str<S: AsRef<str>>(s: S) -> Result<Self, Self::Error>;

    fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error>;
}
