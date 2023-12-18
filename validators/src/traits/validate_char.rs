/// Validate and deserialize characters.
pub trait ValidateChar: Sized {
    type Error;

    fn parse_char(c: char) -> Result<Self, Self::Error>;
    fn validate_char(c: char) -> Result<(), Self::Error>;
}
