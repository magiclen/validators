/// Validate and deserialize (floating) numbers.
pub trait ValidateNumber: Sized {
    type Error;

    /// Whether the inner value is `f32`, so `validators::Result` can read integers and binary formats as `f32`.
    #[doc(hidden)]
    const __VALIDATORS_IS_F32: bool = false;

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
