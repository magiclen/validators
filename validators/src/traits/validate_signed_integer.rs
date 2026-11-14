/// Validate and deserialize signed integers.
pub trait ValidateSignedInteger: Sized {
    type Error;

    /// The bit width that serde uses for the inner integer (64 for `isize` and `usize`), or `0` if it is unknown.
    /// `validators::Result` uses it to ask binary formats for the exact type.
    #[doc(hidden)]
    const __VALIDATORS_BITS: u32 = 0;

    fn parse_i128(i: i128) -> Result<Self, Self::Error>;

    fn validate_i128(i: i128) -> Result<(), Self::Error>;

    #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "16",)))]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self, Self::Error> {
        Self::parse_i64(i as i64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self, Self::Error> {
        Self::parse_i32(i as i32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn parse_isize(i: isize) -> Result<Self, Self::Error> {
        Self::parse_i16(i as i16)
    }

    #[inline]
    fn parse_i64(i: i64) -> Result<Self, Self::Error> {
        Self::parse_i128(i as i128)
    }

    #[inline]
    fn parse_i32(i: i32) -> Result<Self, Self::Error> {
        Self::parse_i64(i as i64)
    }

    #[inline]
    fn parse_i16(i: i16) -> Result<Self, Self::Error> {
        Self::parse_i32(i as i32)
    }

    #[inline]
    fn parse_i8(i: i8) -> Result<Self, Self::Error> {
        Self::parse_i16(i as i16)
    }

    #[cfg(not(any(target_pointer_width = "32", target_pointer_width = "16",)))]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i64(i as i64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i32(i as i32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn validate_isize(i: isize) -> Result<(), Self::Error> {
        Self::validate_i16(i as i16)
    }

    #[inline]
    fn validate_i64(i: i64) -> Result<(), Self::Error> {
        Self::validate_i128(i as i128)
    }

    #[inline]
    fn validate_i32(i: i32) -> Result<(), Self::Error> {
        Self::validate_i64(i as i64)
    }

    #[inline]
    fn validate_i16(i: i16) -> Result<(), Self::Error> {
        Self::validate_i32(i as i32)
    }

    #[inline]
    fn validate_i8(i: i8) -> Result<(), Self::Error> {
        Self::validate_i16(i as i16)
    }
}
