mod built_in_traits;
#[cfg(feature = "rocket")]
mod rocket_traits;
#[cfg(feature = "serde")]
mod serde_traits;

use core::marker::PhantomData;

/// A wrapper of `core::result::Result`, utilized for specific purposes.
///
/// * This struct uses the `FromParam` trait to implement the `FromFormField` trait (only impl the `from_value` method), allowing it to serve as the error type for subsequent checks.
/// * This struct implements the `Deserialize` trait, allowing it to serve as the error type for subsequent checks.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Result<T, E, K = ()>(core::result::Result<T, E>, PhantomData<K>);

impl<T, E, K> Result<T, E, K> {
    /// Create a new instance.
    #[inline]
    pub const fn new(result: core::result::Result<T, E>) -> Self {
        Self(result, PhantomData)
    }

    /// Convert this instance into `core::result::Result`.
    #[inline]
    pub fn into_std_result(self) -> core::result::Result<T, E> {
        self.0
    }

    /// Get the reference of the `core::result::Result` instance inside this.
    #[inline]
    pub const fn as_std_result(&self) -> &core::result::Result<T, E> {
        &self.0
    }
}
