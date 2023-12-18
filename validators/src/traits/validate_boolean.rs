/// Validate and deserialize booleans.
pub trait ValidateBoolean: Sized {
    type Error;

    fn parse_bool(b: bool) -> Result<Self, Self::Error>;

    fn validate_bool(b: bool) -> Result<(), Self::Error>;
}
