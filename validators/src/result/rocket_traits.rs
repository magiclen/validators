use core::marker::PhantomData;

use rocket::{
    form::{self, FromFormField, ValueField},
    request::FromParam,
};

use super::Result;

impl<'r, T, E> FromFormField<'r> for Result<T, E>
where
    T: FromParam<'r, Error = E> + Send,
    E: Send,
{
    #[inline]
    fn from_value(v: ValueField<'r>) -> form::Result<'r, Self> {
        Ok(Self(<T as FromParam>::from_param(v.value), PhantomData))
    }
}
