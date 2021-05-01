use crate::rocket;

use std::ops::Deref;

use rocket::form::{self, FromFormField, ValueField};
use rocket::request::FromParam;

/// A wrapper of `std::result::Result`, in order to implement the `FromFormField` trait of Rocket.
///
/// This struct uses the `FromParam` trait to implement the `FromFormField` trait (only impl the `from_value` method) so that it can remain the error type for checking later.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Result<T, E>(std::result::Result<T, E>);

impl<T, E> Result<T, E> {
    #[inline]
    pub fn into_std_result(self) -> std::result::Result<T, E> {
        self.0
    }

    #[inline]
    pub fn as_std_result(&self) -> &std::result::Result<T, E> {
        &self.0
    }
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    #[inline]
    fn from(result: std::result::Result<T, E>) -> Self {
        Self(result)
    }
}

impl<T, E> From<Result<T, E>> for std::result::Result<T, E> {
    #[inline]
    fn from(result: Result<T, E>) -> Self {
        result.0
    }
}

impl<T, E> Deref for Result<T, E> {
    type Target = std::result::Result<T, E>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'v, E: Sized + Send, T: FromParam<'v, Error = E> + Send> FromFormField<'v>
    for Result<T, T::Error>
{
    #[inline]
    fn from_value(v: ValueField<'v>) -> form::Result<'v, Self> {
        Ok(<T as FromParam>::from_param(v.value).into())
    }
}
