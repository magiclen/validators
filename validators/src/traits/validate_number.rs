/// Validate and deserialize (floating) numbers.
pub trait ValidateNumber: Sized {
    type Error;

    fn parse_f64(f: f64) -> Result<Self, Self::Error>;

    fn validate_f64(f: f64) -> Result<(), Self::Error>;

    #[inline]
    fn parse_f32(f: f32) -> Result<Self, Self::Error> {
        Self::parse_f64(f as f64)
    }

    #[inline]
    fn validate_f32(f: f32) -> Result<(), Self::Error> {
        Self::validate_f64(f as f64)
    }
}
