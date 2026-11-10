#[cfg(any(feature = "base32", feature = "base64", feature = "base64_url"))]
use alloc::{borrow::Cow, string::String};

enum Error {
    Invalid,
    PaddingMust,
    PaddingDisallow,
}

trait Alphabet {
    const BLOCK_LEN: usize;

    // Only ASCII bytes may return a value, and `parse` relies on this.
    fn value(byte: u8) -> Option<u8>;

    fn tail_mask(length: usize) -> Option<u8>;
}

#[cfg(any(feature = "base32", feature = "base32_decoded"))]
struct Base32;

#[cfg(any(feature = "base32", feature = "base32_decoded"))]
impl Alphabet for Base32 {
    const BLOCK_LEN: usize = 8;

    #[inline]
    fn value(byte: u8) -> Option<u8> {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'2'..=b'7' => Some(byte - b'2' + 26),
            _ => None,
        }
    }

    #[inline]
    fn tail_mask(length: usize) -> Option<u8> {
        match length {
            2 => Some(0b11),
            4 => Some(0b1111),
            5 => Some(0b1),
            7 => Some(0b111),
            _ => None,
        }
    }
}

#[cfg(any(
    feature = "base64",
    feature = "base64_decoded",
    feature = "base64_url",
    feature = "base64_url_decoded"
))]
struct Base64<const URL_SAFE: bool>;

#[cfg(any(
    feature = "base64",
    feature = "base64_decoded",
    feature = "base64_url",
    feature = "base64_url_decoded"
))]
impl<const URL_SAFE: bool> Alphabet for Base64<URL_SAFE> {
    const BLOCK_LEN: usize = 4;

    #[inline]
    fn value(byte: u8) -> Option<u8> {
        match byte {
            b'A'..=b'Z' => Some(byte - b'A'),
            b'a'..=b'z' => Some(byte - b'a' + 26),
            b'0'..=b'9' => Some(byte - b'0' + 52),
            b'+' if !URL_SAFE => Some(62),
            b'/' if !URL_SAFE => Some(63),
            b'-' if URL_SAFE => Some(62),
            b'_' if URL_SAFE => Some(63),
            _ => None,
        }
    }

    #[inline]
    fn tail_mask(length: usize) -> Option<u8> {
        match length {
            2 => Some(0b1111),
            3 => Some(0b11),
            _ => None,
        }
    }
}

#[inline]
fn validate<A: Alphabet, const PADDING_MUST: bool, const PADDING_DISALLOW: bool>(
    bytes: &[u8],
) -> Result<(), Error> {
    if bytes.is_empty() {
        return Err(Error::Invalid);
    }

    let tail_len = (bytes.len() - 1) % A::BLOCK_LEN + 1;
    let mask = if tail_len == A::BLOCK_LEN {
        0
    } else {
        if PADDING_MUST {
            // `PaddingMust` promises that there is no padding, so an input with `=` is invalid instead.
            return Err(if bytes.contains(&b'=') { Error::Invalid } else { Error::PaddingMust });
        }

        A::tail_mask(tail_len).ok_or(Error::Invalid)?
    };

    let (body, tail) = bytes.split_at(bytes.len() - tail_len);

    for &byte in body {
        if A::value(byte).is_none() {
            return Err(Error::Invalid);
        }
    }

    let mut last_value = 0;

    for (index, &byte) in tail.iter().enumerate() {
        if let Some(value) = A::value(byte) {
            last_value = value;
        } else if byte == b'=' {
            if PADDING_DISALLOW {
                return Err(Error::PaddingDisallow);
            }

            let mask = A::tail_mask(index).ok_or(Error::Invalid)?;

            if tail_len != A::BLOCK_LEN || tail[index + 1..].iter().any(|&byte| byte != b'=') {
                return Err(Error::Invalid);
            }

            return check_tail_bits(last_value, mask);
        } else {
            return Err(Error::Invalid);
        }
    }

    check_tail_bits(last_value, mask)
}

#[inline]
fn check_tail_bits(value: u8, mask: u8) -> Result<(), Error> {
    if value & mask == 0 { Ok(()) } else { Err(Error::Invalid) }
}

#[cfg(any(feature = "base32", feature = "base64", feature = "base64_url"))]
#[inline]
fn parse<A: Alphabet, const PADDING_MUST: bool, const PADDING_DISALLOW: bool>(
    bytes: Cow<'_, [u8]>,
) -> Result<String, Error> {
    validate::<A, PADDING_MUST, PADDING_DISALLOW>(&bytes)?;

    // SAFETY: The validator only accepts ASCII bytes.
    Ok(unsafe { String::from_utf8_unchecked(bytes.into_owned()) })
}

macro_rules! export_encoding {
    (
        $module:ident,
        $alphabet:ty,
        $feature:literal,
        $error:ident,
        $decoded_feature:literal,
        $decoded_error:ident
    ) => {
        pub mod $module {
            use super::Error;
            #[cfg(feature = $feature)]
            use super::{Cow, String};
            #[cfg(feature = $decoded_feature)]
            use crate::errors::$decoded_error;
            #[cfg(feature = $feature)]
            use crate::errors::$error;

            #[cfg(feature = $feature)]
            #[inline]
            fn map_error(error: Error) -> $error {
                match error {
                    Error::Invalid => $error::Invalid,
                    Error::PaddingMust => $error::PaddingMust,
                    Error::PaddingDisallow => $error::PaddingDisallow,
                }
            }

            #[cfg(feature = $feature)]
            #[inline]
            pub fn validate<const PADDING_MUST: bool, const PADDING_DISALLOW: bool>(
                bytes: &[u8],
            ) -> Result<(), $error> {
                super::validate::<$alphabet, PADDING_MUST, PADDING_DISALLOW>(bytes)
                    .map_err(map_error)
            }

            #[cfg(feature = $feature)]
            #[inline]
            pub fn parse<const PADDING_MUST: bool, const PADDING_DISALLOW: bool>(
                bytes: Cow<'_, [u8]>,
            ) -> Result<String, $error> {
                super::parse::<$alphabet, PADDING_MUST, PADDING_DISALLOW>(bytes).map_err(map_error)
            }

            #[cfg(feature = $decoded_feature)]
            #[inline]
            fn map_decoded_error(error: Error) -> $decoded_error {
                match error {
                    Error::Invalid => $decoded_error::Invalid,
                    Error::PaddingMust => $decoded_error::PaddingMust,
                    Error::PaddingDisallow => $decoded_error::PaddingDisallow,
                }
            }

            #[cfg(feature = $decoded_feature)]
            #[inline]
            pub fn validate_decoded<const PADDING_MUST: bool, const PADDING_DISALLOW: bool>(
                bytes: &[u8],
            ) -> Result<(), $decoded_error> {
                super::validate::<$alphabet, PADDING_MUST, PADDING_DISALLOW>(bytes)
                    .map_err(map_decoded_error)
            }
        }
    };
}

#[cfg(any(feature = "base32", feature = "base32_decoded"))]
export_encoding!(
    base32,
    super::Base32,
    "base32",
    Base32Error,
    "base32_decoded",
    Base32DecodedError
);
#[cfg(any(feature = "base64", feature = "base64_decoded"))]
export_encoding!(
    base64,
    super::Base64<false>,
    "base64",
    Base64Error,
    "base64_decoded",
    Base64DecodedError
);
#[cfg(any(feature = "base64_url", feature = "base64_url_decoded"))]
export_encoding!(
    base64_url,
    super::Base64<true>,
    "base64_url",
    Base64UrlError,
    "base64_url_decoded",
    Base64UrlDecodedError
);
