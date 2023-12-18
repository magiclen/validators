#[allow(unused)]
use alloc::{string::String, vec::Vec};
#[allow(unused)]
use core::{
    fmt::{self, Formatter},
    marker::PhantomData,
};

#[allow(unused)]
use serde::de::{Deserializer, Error as DeError, Visitor};

#[allow(unused)]
use super::Result;
#[allow(unused)]
use crate::{errors::*, traits::*};

#[cfg(feature = "base32")]
impl<'de, T: ValidateString<Error = Base32Error> + ValidateBytes<Error = Base32Error>>
    serde::Deserialize<'de> for Result<T, Base32Error>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = Base32Error> + ValidateBytes<Error = Base32Error>>
            Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, Base32Error>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("Base32Error")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "base32_decoded")]
impl<
        'de,
        T: ValidateString<Error = Base32DecodedError> + ValidateBytes<Error = Base32DecodedError>,
    > serde::Deserialize<'de> for Result<T, Base32DecodedError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<
                'de,
                T: ValidateString<Error = Base32DecodedError>
                    + ValidateBytes<Error = Base32DecodedError>,
            > Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, Base32DecodedError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("Base32DecodedError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "base64")]
impl<'de, T: ValidateString<Error = Base64Error> + ValidateBytes<Error = Base64Error>>
    serde::Deserialize<'de> for Result<T, Base64Error>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = Base64Error> + ValidateBytes<Error = Base64Error>>
            Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, Base64Error>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("Base64Error")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "base64_decoded")]
impl<
        'de,
        T: ValidateString<Error = Base64DecodedError> + ValidateBytes<Error = Base64DecodedError>,
    > serde::Deserialize<'de> for Result<T, Base64DecodedError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<
                'de,
                T: ValidateString<Error = Base64DecodedError>
                    + ValidateBytes<Error = Base64DecodedError>,
            > Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, Base64DecodedError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("Base64DecodedError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "base64_url")]
impl<'de, T: ValidateString<Error = Base64UrlError> + ValidateBytes<Error = Base64UrlError>>
    serde::Deserialize<'de> for Result<T, Base64UrlError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<
                'de,
                T: ValidateString<Error = Base64UrlError> + ValidateBytes<Error = Base64UrlError>,
            > Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, Base64UrlError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("Base64UrlError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "base64_url_decoded")]
impl<
        'de,
        T: ValidateString<Error = Base64UrlDecodedError>
            + ValidateBytes<Error = Base64UrlDecodedError>,
    > serde::Deserialize<'de> for Result<T, Base64UrlDecodedError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<
                'de,
                T: ValidateString<Error = Base64UrlDecodedError>
                    + ValidateBytes<Error = Base64UrlDecodedError>,
            > Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, Base64UrlDecodedError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("Base64UrlDecodedError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_bytes<E>(self, v: &[u8]) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u8_slice(v)))
            }

            #[inline]
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_vec_u8(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "bit")]
impl<'de, T: ValidateString<Error = BitError> + ValidateUnsignedInteger<Error = BitError>>
    serde::Deserialize<'de> for Result<T, BitError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;

            Ok(Result::new(T::parse_string(s)))
        } else {
            let s = u128::deserialize(deserializer)?;

            Ok(Result::new(T::parse_u128(s)))
        }
    }
}

#[cfg(feature = "boolean")]
impl<
        'de,
        T: ValidateString<Error = BooleanError>
            + ValidateChar<Error = BooleanError>
            + ValidateSignedInteger<Error = BooleanError>
            + ValidateUnsignedInteger<Error = BooleanError>
            + ValidateBoolean<Error = BooleanError>,
    > serde::Deserialize<'de> for Result<T, BooleanError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<
                'de,
                T: ValidateString<Error = BooleanError>
                    + ValidateChar<Error = BooleanError>
                    + ValidateSignedInteger<Error = BooleanError>
                    + ValidateUnsignedInteger<Error = BooleanError>
                    + ValidateBoolean<Error = BooleanError>,
            > Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, BooleanError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("BooleanError")
            }

            fn visit_bool<E>(self, v: bool) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_bool(v)))
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_i64(v)))
            }

            #[inline]
            fn visit_i128<E>(self, v: i128) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_i128(v)))
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u64(v)))
            }

            #[inline]
            fn visit_u128<E>(self, v: u128) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u128(v)))
            }

            #[inline]
            fn visit_char<E>(self, v: char) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_char(v)))
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "byte")]
impl<'de, T: ValidateString<Error = ByteError> + ValidateUnsignedInteger<Error = ByteError>>
    serde::Deserialize<'de> for Result<T, ByteError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        if deserializer.is_human_readable() {
            let s = String::deserialize(deserializer)?;

            Ok(Result::new(T::parse_string(s)))
        } else {
            let s = u128::deserialize(deserializer)?;

            Ok(Result::new(T::parse_u128(s)))
        }
    }
}

#[cfg(feature = "domain")]
impl<'de, T: ValidateString<Error = DomainError>> serde::Deserialize<'de>
    for Result<T, DomainError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = DomainError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, DomainError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("DomainError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "email")]
impl<'de, T: ValidateString<Error = EmailError>> serde::Deserialize<'de> for Result<T, EmailError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = EmailError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, EmailError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("EmailError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "host")]
impl<'de, T: ValidateString<Error = HostError>> serde::Deserialize<'de> for Result<T, HostError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = HostError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, HostError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("HostError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "http_url")]
impl<'de, T: ValidateString<Error = HttpURLError>> serde::Deserialize<'de>
    for Result<T, HttpURLError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = HttpURLError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, HttpURLError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("HttpURLError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "http_ftp_url")]
impl<'de, T: ValidateString<Error = HttpFtpURLError>> serde::Deserialize<'de>
    for Result<T, HttpFtpURLError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = HttpFtpURLError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, HttpFtpURLError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("HttpFtpURLError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "ip")]
impl<'de, T: ValidateString<Error = IpError>> serde::Deserialize<'de> for Result<T, IpError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = IpError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, IpError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("IpError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "ipv4")]
impl<'de, T: ValidateString<Error = Ipv4Error>> serde::Deserialize<'de> for Result<T, Ipv4Error> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = Ipv4Error>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, Ipv4Error>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("Ipv4Error")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "ipv6")]
impl<'de, T: ValidateString<Error = Ipv6Error>> serde::Deserialize<'de> for Result<T, Ipv6Error> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = Ipv6Error>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, Ipv6Error>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("Ipv6Error")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "json")]
impl<
        'de,
        T: ValidateString<Error = JsonError>
            + ValidateSignedInteger<Error = JsonError>
            + ValidateUnsignedInteger<Error = JsonError>
            + ValidateNumber<Error = JsonError>
            + ValidateBoolean<Error = JsonError>
            + ValidateJsonValue<Error = JsonError>,
    > serde::Deserialize<'de> for Result<T, JsonError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        match serde_json::Value::deserialize(deserializer) {
            Ok(value) => Ok(Result::new(T::parse_json_value(value))),
            Err(_) => Ok(Result::new(Err(JsonError::InvalidJsonValueError))),
        }
    }
}

#[cfg(feature = "length")]
impl<
        'de,
        C: CollectionLength + serde::Deserialize<'de>,
        T: ValidateLength<C, Error = LengthError>,
    > serde::Deserialize<'de> for Result<T, LengthError, C>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        Ok(Result::new(T::parse_collection(serde::Deserialize::deserialize(deserializer)?)))
    }
}

#[cfg(feature = "line")]
impl<'de, T: ValidateString<Error = LineError>> serde::Deserialize<'de> for Result<T, LineError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = LineError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, LineError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("LineError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_string(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "mac_address")]
impl<'de, T: ValidateString<Error = MacAddressError>> serde::Deserialize<'de>
    for Result<T, MacAddressError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = MacAddressError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, MacAddressError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("MacAddressError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "number")]
impl<'de, T: ValidateString<Error = NumberError> + ValidateNumber<Error = NumberError>>
    serde::Deserialize<'de> for Result<T, NumberError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = NumberError> + ValidateNumber<Error = NumberError>>
            Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, NumberError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("NumberError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_f32<E>(self, v: f32) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_f32(v)))
            }

            #[inline]
            fn visit_f64<E>(self, v: f64) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_f64(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "phone")]
impl<'de, T: ValidateString<Error = PhoneError>> serde::Deserialize<'de> for Result<T, PhoneError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = PhoneError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, PhoneError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("PhoneError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "regex")]
impl<'de, T: ValidateString<Error = RegexError>> serde::Deserialize<'de> for Result<T, RegexError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = RegexError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, RegexError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("RegexError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_string(MyVisitor(PhantomData))
    }
}

#[cfg(any(feature = "semver", feature = "semver_req"))]
impl<'de, T: ValidateString<Error = SemverError>> serde::Deserialize<'de>
    for Result<T, SemverError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = SemverError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, SemverError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("SemverError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "signed_integer")]
impl<
        'de,
        T: ValidateString<Error = SignedIntegerError>
            + ValidateSignedInteger<Error = SignedIntegerError>,
    > serde::Deserialize<'de> for Result<T, SignedIntegerError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<
                'de,
                T: ValidateString<Error = SignedIntegerError>
                    + ValidateSignedInteger<Error = SignedIntegerError>,
            > Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, SignedIntegerError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("SignedIntegerError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_i8<E>(self, v: i8) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_i8(v)))
            }

            #[inline]
            fn visit_i16<E>(self, v: i16) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_i16(v)))
            }

            #[inline]
            fn visit_i32<E>(self, v: i32) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_i32(v)))
            }

            #[inline]
            fn visit_i64<E>(self, v: i64) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_i64(v)))
            }

            #[inline]
            fn visit_i128<E>(self, v: i128) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_i128(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "text")]
impl<'de, T: ValidateString<Error = TextError>> serde::Deserialize<'de> for Result<T, TextError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = TextError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, TextError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("TextError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }
        }

        deserializer.deserialize_string(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "unsigned_integer")]
impl<
        'de,
        T: ValidateString<Error = UnsignedIntegerError>
            + ValidateUnsignedInteger<Error = UnsignedIntegerError>,
    > serde::Deserialize<'de> for Result<T, UnsignedIntegerError>
{
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<
                'de,
                T: ValidateString<Error = UnsignedIntegerError>
                    + ValidateUnsignedInteger<Error = UnsignedIntegerError>,
            > Visitor<'de> for MyVisitor<T>
        {
            type Value = Result<T, UnsignedIntegerError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("UnsignedIntegerError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }

            #[inline]
            fn visit_string<E>(self, v: String) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_string(v)))
            }

            #[inline]
            fn visit_u8<E>(self, v: u8) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u8(v)))
            }

            #[inline]
            fn visit_u16<E>(self, v: u16) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u16(v)))
            }

            #[inline]
            fn visit_u32<E>(self, v: u32) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u32(v)))
            }

            #[inline]
            fn visit_u64<E>(self, v: u64) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u64(v)))
            }

            #[inline]
            fn visit_u128<E>(self, v: u128) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_u128(v)))
            }
        }

        deserializer.deserialize_any(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "url")]
impl<'de, T: ValidateString<Error = UrlError>> serde::Deserialize<'de> for Result<T, UrlError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = UrlError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, UrlError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("UrlError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}

#[cfg(feature = "uuid")]
impl<'de, T: ValidateString<Error = UuidError>> serde::Deserialize<'de> for Result<T, UuidError> {
    #[inline]
    fn deserialize<D>(deserializer: D) -> core::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>, {
        struct MyVisitor<T>(PhantomData<T>);

        impl<'de, T: ValidateString<Error = UuidError>> Visitor<'de> for MyVisitor<T> {
            type Value = Result<T, UuidError>;

            #[inline]
            fn expecting(&self, f: &mut Formatter) -> fmt::Result {
                f.write_str("UuidError")
            }

            #[inline]
            fn visit_str<E>(self, v: &str) -> core::result::Result<Self::Value, E>
            where
                E: DeError, {
                Ok(Result::new(T::parse_str(v)))
            }
        }

        deserializer.deserialize_str(MyVisitor(PhantomData))
    }
}
