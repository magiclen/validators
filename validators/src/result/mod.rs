use crate::rocket;

use rocket::form::{self, FromFormField, ValueField};
use rocket::request::FromParam;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E> {
    #[inline]
    fn from(result: std::result::Result<T, E>) -> Self {
        match result {
            Ok(v) => Result::Ok(v),
            Err(v) => Result::Err(v),
        }
    }
}

impl<'v, E: Sized + Send, T: FromParam<'v, Error = E> + Send> FromFormField<'v>
    for Result<T, T::Error>
{
    #[inline]
    fn from_value(v: ValueField<'v>) -> form::Result<'v, Self> {
        Ok(Result::from(<T as FromParam>::from_param(v.value)))
    }
}
