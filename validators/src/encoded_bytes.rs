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

#[cfg(feature = "base32")]
struct Base32;

#[cfg(feature = "base32")]
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

#[cfg(any(feature = "base64", feature = "base64_url"))]
struct Base64<const URL_SAFE: bool>;

#[cfg(any(feature = "base64", feature = "base64_url"))]
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
            return Err(Error::PaddingMust);
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

#[inline]
fn parse<A: Alphabet, const PADDING_MUST: bool, const PADDING_DISALLOW: bool>(
    bytes: Cow<'_, [u8]>,
) -> Result<String, Error> {
    validate::<A, PADDING_MUST, PADDING_DISALLOW>(&bytes)?;

    // SAFETY: The validator only accepts ASCII bytes.
    Ok(unsafe { String::from_utf8_unchecked(bytes.into_owned()) })
}

macro_rules! export_encoding {
    ($module:ident, $alphabet:ty, $error:ident) => {
        pub mod $module {
            use super::{Cow, Error, String};
            use crate::errors::$error;

            #[inline]
            fn map_error(error: Error) -> $error {
                match error {
                    Error::Invalid => $error::Invalid,
                    Error::PaddingMust => $error::PaddingMust,
                    Error::PaddingDisallow => $error::PaddingDisallow,
                }
            }

            #[inline]
            pub fn validate<const PADDING_MUST: bool, const PADDING_DISALLOW: bool>(
                bytes: &[u8],
            ) -> Result<(), $error> {
                super::validate::<$alphabet, PADDING_MUST, PADDING_DISALLOW>(bytes)
                    .map_err(map_error)
            }

            #[inline]
            pub fn parse<const PADDING_MUST: bool, const PADDING_DISALLOW: bool>(
                bytes: Cow<'_, [u8]>,
            ) -> Result<String, $error> {
                super::parse::<$alphabet, PADDING_MUST, PADDING_DISALLOW>(bytes).map_err(map_error)
            }
        }
    };
}

#[cfg(feature = "base32")]
export_encoding!(base32, super::Base32, Base32Error);
#[cfg(feature = "base64")]
export_encoding!(base64, super::Base64<false>, Base64Error);
#[cfg(feature = "base64_url")]
export_encoding!(base64_url, super::Base64<true>, Base64UrlError);
