use alloc::vec::Vec;

/// Validate and deserialize bytes.
pub trait ValidateBytes: Sized {
    type Error;

    fn parse_vec_u8<V: Into<Vec<u8>>>(v: V) -> Result<Self, Self::Error>;

    fn parse_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<Self, Self::Error>;

    fn validate_u8_slice<V: AsRef<[u8]>>(v: V) -> Result<(), Self::Error>;

    #[doc(hidden)]
    #[inline]
    fn parse_decoded_vec_u8(v: Vec<u8>) -> Result<Self, Self::Error> {
        Self::parse_vec_u8(v)
    }
}
