#![allow(unexpected_cfgs)] // pointer widths intentional
/// Validate and deserialize unsigned integers.
pub trait ValidateUnsignedInteger: Sized {
    type Error;

    fn parse_u128(u: u128) -> Result<Self, Self::Error>;

    fn validate_u128(u: u128) -> Result<(), Self::Error>;

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self, Self::Error> {
        Self::parse_u128(u as u128)
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self, Self::Error> {
        Self::parse_u64(u as u64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self, Self::Error> {
        Self::parse_u32(u as u32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self, Self::Error> {
        Self::parse_u16(u as u16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn parse_usize(u: usize) -> Result<Self, Self::Error> {
        Self::parse_u8(u as u8)
    }

    #[inline]
    fn parse_u64(u: u64) -> Result<Self, Self::Error> {
        Self::parse_u128(u as u128)
    }

    #[inline]
    fn parse_u32(u: u32) -> Result<Self, Self::Error> {
        Self::parse_u64(u as u64)
    }

    #[inline]
    fn parse_u16(u: u16) -> Result<Self, Self::Error> {
        Self::parse_u32(u as u32)
    }

    #[inline]
    fn parse_u8(u: u8) -> Result<Self, Self::Error> {
        Self::parse_u16(u as u16)
    }

    #[cfg(target_pointer_width = "128")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u128(u as u128)
    }

    #[cfg(not(any(
        target_pointer_width = "128",
        target_pointer_width = "32",
        target_pointer_width = "16",
        target_pointer_width = "8"
    )))]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u64(u as u64)
    }

    #[cfg(target_pointer_width = "32")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u32(u as u32)
    }

    #[cfg(target_pointer_width = "16")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u16(u as u16)
    }

    #[cfg(target_pointer_width = "8")]
    #[inline]
    fn validate_usize(u: usize) -> Result<(), Self::Error> {
        Self::validate_u8(u as u8)
    }

    #[inline]
    fn validate_u64(u: u64) -> Result<(), Self::Error> {
        Self::validate_u128(u as u128)
    }

    #[inline]
    fn validate_u32(u: u32) -> Result<(), Self::Error> {
        Self::validate_u64(u as u64)
    }

    #[inline]
    fn validate_u16(u: u16) -> Result<(), Self::Error> {
        Self::validate_u32(u as u32)
    }

    #[inline]
    fn validate_u8(u: u8) -> Result<(), Self::Error> {
        Self::validate_u16(u as u16)
    }
}
