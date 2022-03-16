use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

#[cfg(feature = "rocket")]
use rocket::form::{self, FromFormField, ValueField};
#[cfg(feature = "rocket")]
use rocket::request::FromParam;

/// A wrapper of `std::result::Result`, in order to implement the `FromFormField` trait of Rocket and the `Deserialize` trait of serde.
///
/// This struct uses the `FromParam` trait to implement the `FromFormField` trait (only impl the `from_value` method) so that it can remain the error type for checking later.
/// This struct implements the `Deserialize` trait so that it can remain the error type for checking later.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Result<T, E, D = ()>(std::result::Result<T, E>, PhantomData<D>);

impl<T, E> Result<T, E> {
    #[inline]
    pub fn new(result: std::result::Result<T, E>) -> Self {
        Self(result, PhantomData)
    }

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
        Self::new(result)
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

impl<T, E> DerefMut for Result<T, E> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(feature = "rocket")]
impl<'v, E: Sized + Send, T: FromParam<'v, Error = E> + Send> FromFormField<'v>
    for Result<T, T::Error>
{
    #[inline]
    fn from_value(v: ValueField<'v>) -> form::Result<'v, Self> {
        Ok(<T as FromParam>::from_param(v.value).into())
    }
}

#[cfg(all(feature = "serde", feature = "base32"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::Base32Error>
            + super::traits::ValidateBytes<Output = T, Error = super::errors::Base32Error>,
    > serde::Deserialize<'de> for Result<T, super::errors::Base32Error>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::Base32Error>
                    + super::traits::ValidateBytes<Output = T, Error = super::errors::Base32Error>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::Base32Error>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("Base32Error")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(
                self,
                v: alloc::vec::Vec<u8>,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "base32_decoded"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::Base32DecodedError>
            + super::traits::ValidateBytes<Output = T, Error = super::errors::Base32DecodedError>,
    > serde::Deserialize<'de> for Result<T, super::errors::Base32DecodedError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<
                        Output = T,
                        Error = super::errors::Base32DecodedError,
                    > + super::traits::ValidateBytes<
                        Output = T,
                        Error = super::errors::Base32DecodedError,
                    >,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::Base32DecodedError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("Base32DecodedError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(
                self,
                v: alloc::vec::Vec<u8>,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "base64"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::Base64Error>
            + super::traits::ValidateBytes<Output = T, Error = super::errors::Base64Error>,
    > serde::Deserialize<'de> for Result<T, super::errors::Base64Error>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::Base64Error>
                    + super::traits::ValidateBytes<Output = T, Error = super::errors::Base64Error>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::Base64Error>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("Base64Error")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(
                self,
                v: alloc::vec::Vec<u8>,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "base64_decoded"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::Base64DecodedError>
            + super::traits::ValidateBytes<Output = T, Error = super::errors::Base64DecodedError>,
    > serde::Deserialize<'de> for Result<T, super::errors::Base64DecodedError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<
                        Output = T,
                        Error = super::errors::Base64DecodedError,
                    > + super::traits::ValidateBytes<
                        Output = T,
                        Error = super::errors::Base64DecodedError,
                    >,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::Base64DecodedError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("Base64DecodedError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(
                self,
                v: alloc::vec::Vec<u8>,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "base64_url"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::Base64UrlError>
            + super::traits::ValidateBytes<Output = T, Error = super::errors::Base64UrlError>,
    > serde::Deserialize<'de> for Result<T, super::errors::Base64UrlError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::Base64UrlError>
                    + super::traits::ValidateBytes<Output = T, Error = super::errors::Base64UrlError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::Base64UrlError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("Base64UrlError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(
                self,
                v: alloc::vec::Vec<u8>,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "base64_url_decoded"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::Base64UrlDecodedError>
            + super::traits::ValidateBytes<Output = T, Error = super::errors::Base64UrlDecodedError>,
    > serde::Deserialize<'de> for Result<T, super::errors::Base64UrlDecodedError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<
                        Output = T,
                        Error = super::errors::Base64UrlDecodedError,
                    > + super::traits::ValidateBytes<
                        Output = T,
                        Error = super::errors::Base64UrlDecodedError,
                    >,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::Base64UrlDecodedError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("Base64UrlDecodedError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(
                self,
                v: alloc::vec::Vec<u8>,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "boolean"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::BooleanError>
            + super::traits::ValidateChar<Output = T, Error = super::errors::BooleanError>
            + super::traits::ValidateSignedInteger<Output = T, Error = super::errors::BooleanError>
            + super::traits::ValidateUnsignedInteger<Output = T, Error = super::errors::BooleanError>
            + super::traits::ValidateBoolean<Output = T, Error = super::errors::BooleanError>,
    > serde::Deserialize<'de> for Result<T, super::errors::BooleanError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::BooleanError>
                    + super::traits::ValidateChar<Output = T, Error = super::errors::BooleanError>
                    + super::traits::ValidateSignedInteger<
                        Output = T,
                        Error = super::errors::BooleanError,
                    > + super::traits::ValidateUnsignedInteger<
                        Output = T,
                        Error = super::errors::BooleanError,
                    > + super::traits::ValidateBoolean<Output = T, Error = super::errors::BooleanError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::BooleanError>;

            serde::serde_if_integer128! {
                #[inline]
                fn visit_i128<E>(self, v: i128) -> core::result::Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Result::new(T::parse_i128(v)))
                }

                #[inline]
                fn visit_u128<E>(self, v: u128) -> core::result::Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Result::new(T::parse_u128(v)))
                }
            }

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("BooleanError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            fn visit_bool<E>(self, v: bool) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_bool(v)))
            }

            #[inline]
            fn visit_char<E>(self, v: char) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_char(v)))
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_i64(v)))
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_u64(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "domain"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::DomainError>>
    serde::Deserialize<'de> for Result<T, super::errors::DomainError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::DomainError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::DomainError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("DomainError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "email"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::EmailError>>
    serde::Deserialize<'de> for Result<T, super::errors::EmailError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::EmailError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::EmailError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("EmailError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "host"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::HostError>>
    serde::Deserialize<'de> for Result<T, super::errors::HostError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::HostError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::HostError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("HostError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "http_url"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::HttpURLError>>
    serde::Deserialize<'de> for Result<T, super::errors::HttpURLError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::HttpURLError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::HttpURLError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("HttpURLError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "http_ftp_url"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::HttpFtpURLError>>
    serde::Deserialize<'de> for Result<T, super::errors::HttpFtpURLError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::HttpFtpURLError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::HttpFtpURLError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("HttpFtpURLError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "ip"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::IPError>>
    serde::Deserialize<'de> for Result<T, super::errors::IPError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::IPError>>
            serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::IPError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("IPError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "ipv4"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::IPv4Error>>
    serde::Deserialize<'de> for Result<T, super::errors::IPv4Error>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::IPv4Error>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::IPv4Error>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("IPv4Error")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "ipv6"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::IPv6Error>>
    serde::Deserialize<'de> for Result<T, super::errors::IPv6Error>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::IPv6Error>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::IPv6Error>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("IPv6Error")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "json"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::JSONError>
            + super::traits::ValidateSignedInteger<Output = T, Error = super::errors::JSONError>
            + super::traits::ValidateUnsignedInteger<Output = T, Error = super::errors::JSONError>
            + super::traits::ValidateNumber<Output = T, Error = super::errors::JSONError>
            + super::traits::ValidateBoolean<Output = T, Error = super::errors::JSONError>
            + super::traits::ValidateJsonValue<Output = T, Error = super::errors::JSONError>,
    > serde::Deserialize<'de> for Result<T, super::errors::JSONError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        match serde_json::Value::deserialize(deserializer) {
            Ok(value) => Ok(Result::new(T::parse_json_value(value))),
            Err(_) => Ok(Result::new(Err(super::errors::JSONError::InvalidJsonValueError))),
        }
    }
}

#[cfg(all(feature = "serde", feature = "length"))]
impl<
        'de,
        T: super::traits::CollectionLength + serde::Deserialize<'de>,
        K: super::traits::ValidateLength<T, Output = K, Error = super::errors::LengthError>,
    > serde::Deserialize<'de> for Result<K, super::errors::LengthError, T>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        Ok(Result(K::parse_collection(serde::Deserialize::deserialize(deserializer)?), PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "line"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::LineError>>
    serde::Deserialize<'de> for Result<T, super::errors::LineError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::LineError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::LineError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("LineError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "mac_address"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::MacAddressError>>
    serde::Deserialize<'de> for Result<T, super::errors::MacAddressError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::MacAddressError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::MacAddressError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("MacAddressError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "number"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::NumberError>>
    serde::Deserialize<'de> for Result<T, super::errors::NumberError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::NumberError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::NumberError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("NumberError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "phone"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::PhoneError>>
    serde::Deserialize<'de> for Result<T, super::errors::PhoneError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::PhoneError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::PhoneError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("PhoneError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "regex"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::RegexError>>
    serde::Deserialize<'de> for Result<T, super::errors::RegexError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::RegexError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::RegexError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("RegexError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", any(feature = "semver", feature = "semver_req")))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::SemVerError>>
    serde::Deserialize<'de> for Result<T, super::errors::SemVerError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::SemVerError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::SemVerError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("SemVerError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "signed_integer"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::SignedIntegerError>
            + super::traits::ValidateSignedInteger<
                Output = T,
                Error = super::errors::SignedIntegerError,
            >,
    > serde::Deserialize<'de> for Result<T, super::errors::SignedIntegerError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<
                        Output = T,
                        Error = super::errors::SignedIntegerError,
                    > + super::traits::ValidateSignedInteger<
                        Output = T,
                        Error = super::errors::SignedIntegerError,
                    >,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::SignedIntegerError>;

            serde::serde_if_integer128! {
                #[inline]
                fn visit_i128<E>(self, v: i128) -> core::result::Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Result::new(T::parse_i128(v)))
                }
            }

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("SignedIntegerError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_i64(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "text"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::TextError>>
    serde::Deserialize<'de> for Result<T, super::errors::TextError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::TextError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::TextError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("TextError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "unsigned_integer"))]
impl<
        'de,
        T: super::traits::ValidateString<Output = T, Error = super::errors::UnsignedIntegerError>
            + super::traits::ValidateUnsignedInteger<
                Output = T,
                Error = super::errors::UnsignedIntegerError,
            >,
    > serde::Deserialize<'de> for Result<T, super::errors::UnsignedIntegerError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<
                        Output = T,
                        Error = super::errors::UnsignedIntegerError,
                    > + super::traits::ValidateUnsignedInteger<
                        Output = T,
                        Error = super::errors::UnsignedIntegerError,
                    >,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::UnsignedIntegerError>;

            serde::serde_if_integer128! {
                #[inline]
                fn visit_u128<E>(self, v: u128) -> core::result::Result<Self::Value, E>
                where
                    E: serde::de::Error,
                {
                    Ok(Result::new(T::parse_u128(v)))
                }
            }

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("UnsignedIntegerError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_u64(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "url"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::URLError>>
    serde::Deserialize<'de> for Result<T, super::errors::URLError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::URLError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::URLError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("URLError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}

#[cfg(all(feature = "serde", feature = "uuid"))]
impl<'de, T: super::traits::ValidateString<Output = T, Error = super::errors::UUIDError>>
    serde::Deserialize<'de> for Result<T, super::errors::UUIDError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct ValidatingVisitor<T>(core::marker::PhantomData<T>);

        impl<
                'de,
                T: super::traits::ValidateString<Output = T, Error = super::errors::UUIDError>,
            > serde::de::Visitor<'de> for ValidatingVisitor<T>
        {
            type Value = Result<T, super::errors::UUIDError>;

            #[inline]
            fn expecting(
                &self,
                f: &mut core::fmt::Formatter,
            ) -> core::result::Result<(), core::fmt::Error> {
                f.write_str("UUIDError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(
                self,
                v: alloc::string::String,
            ) -> core::result::Result<Self::Value, E>
            where
                E: serde::de::Error, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(ValidatingVisitor(core::marker::PhantomData))
    }
}
