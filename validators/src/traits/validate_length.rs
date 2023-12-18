use crate::traits::CollectionLength;

/// Validate the length of collections.
pub trait ValidateLength<T: CollectionLength>: Sized {
    type Error;

    fn parse_collection(v: T) -> Result<Self, Self::Error>;

    fn validate_collection(v: &T) -> Result<(), Self::Error>;
}
