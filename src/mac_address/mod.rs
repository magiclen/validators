extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper, ValidatorOption};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::Utf8Error;

lazy_static! {
    static ref MAC_ADDRESS_COLON_UPPERCASE_RE: Regex =
        Regex::new("^([0-9A-F]{2}:){5}[0-9A-F]{2}").unwrap();
    static ref MAC_ADDRESS_COLON_LOWERCASE_RE: Regex =
        Regex::new("^([0-9a-f]{2}:){5}[0-9a-f]{2}$").unwrap();
    static ref MAC_ADDRESS_COLON_RE: Regex =
        Regex::new("^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$").unwrap();
    static ref MAC_ADDRESS_NO_COLON_UPPERCASE_RE: Regex = Regex::new("^[0-9A-F]{12}").unwrap();
    static ref MAC_ADDRESS_NO_COLON_LOWERCASE_RE: Regex = Regex::new("^[0-9a-f]{12}$").unwrap();
    static ref MAC_ADDRESS_NO_COLON_RE: Regex = Regex::new("^[0-9a-fA-F]{12}$").unwrap();
}

#[derive(Debug, PartialEq, Clone)]
pub enum MacAddressError {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for MacAddressError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for MacAddressError {}

impl From<Utf8Error> for MacAddressError {
    #[inline]
    #[inline]
    fn from(err: Utf8Error) -> Self {
        MacAddressError::UTF8Error(err)
    }
}

pub type MacAddressResult = Result<MacAddress, MacAddressError>;

#[derive(Debug, PartialEq)]
pub struct MacAddressValidator {
    pub lowercase: ValidatorOption,
    pub colon: ValidatorOption,
}

#[derive(Clone)]
enum MacAddressCase {
    Upper,
    Lower,
    Both,
}

#[derive(Clone)]
pub struct MacAddress {
    mac_address: String,
    case: MacAddressCase,
    colon: bool,
}

impl MacAddress {
    #[inline]
    pub fn get_full_mac_address(&self) -> &str {
        &self.mac_address
    }

    #[inline]
    pub fn has_lowercase(&self) -> bool {
        match self.case {
            MacAddressCase::Upper => false,
            MacAddressCase::Lower => true,
            MacAddressCase::Both => true,
        }
    }

    #[inline]
    pub fn has_uppercase(&self) -> bool {
        match self.case {
            MacAddressCase::Upper => true,
            MacAddressCase::Lower => false,
            MacAddressCase::Both => true,
        }
    }

    #[inline]
    pub fn has_both_case(&self) -> bool {
        match self.case {
            MacAddressCase::Both => true,
            _ => false,
        }
    }

    #[inline]
    pub fn has_colon(&self) -> bool {
        self.colon
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.mac_address
    }
}

impl Deref for MacAddress {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.mac_address
    }
}

impl Validated for MacAddress {}

impl Debug for MacAddress {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(MacAddress, f, self, let .0 = self.mac_address);
    }
}

impl Display for MacAddress {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.mac_address)?;
        Ok(())
    }
}

impl PartialEq for MacAddress {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.mac_address.eq(&other.mac_address)
    }
}

impl Eq for MacAddress {}

impl Hash for MacAddress {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mac_address.hash(state)
    }
}

impl MacAddressValidator {
    #[inline]
    pub fn is_mac_address(&self, mac_address: &str) -> bool {
        self.parse_inner(mac_address).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, mac_address: String) -> MacAddressResult {
        let mut mac_address_inner = self.parse_inner(&mac_address)?;

        mac_address_inner.mac_address = mac_address;

        Ok(mac_address_inner)
    }

    #[inline]
    pub fn parse_str(&self, mac_address: &str) -> MacAddressResult {
        let mut mac_address_inner = self.parse_inner(mac_address)?;

        mac_address_inner.mac_address.push_str(mac_address);

        Ok(mac_address_inner)
    }

    #[allow(clippy::cognitive_complexity)]
    fn parse_inner(&self, mac_address: &str) -> MacAddressResult {
        match self.colon {
            ValidatorOption::Must => {
                if mac_address.len() != 17 {
                    Err(MacAddressError::IncorrectFormat)
                } else {
                    match self.lowercase {
                        ValidatorOption::Must => {
                            if MAC_ADDRESS_COLON_LOWERCASE_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Lower,
                                    colon: true,
                                })
                            } else {
                                Err(MacAddressError::IncorrectFormat)
                            }
                        }
                        ValidatorOption::NotAllow => {
                            if MAC_ADDRESS_COLON_UPPERCASE_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Upper,
                                    colon: true,
                                })
                            } else {
                                Err(MacAddressError::IncorrectFormat)
                            }
                        }
                        ValidatorOption::Allow => {
                            if MAC_ADDRESS_COLON_UPPERCASE_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Upper,
                                    colon: true,
                                })
                            } else if MAC_ADDRESS_COLON_LOWERCASE_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Lower,
                                    colon: true,
                                })
                            } else if MAC_ADDRESS_COLON_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Both,
                                    colon: true,
                                })
                            } else {
                                Err(MacAddressError::IncorrectFormat)
                            }
                        }
                    }
                }
            }
            ValidatorOption::NotAllow => {
                if mac_address.len() != 12 {
                    Err(MacAddressError::IncorrectFormat)
                } else {
                    match self.lowercase {
                        ValidatorOption::Must => {
                            if MAC_ADDRESS_NO_COLON_LOWERCASE_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Lower,
                                    colon: false,
                                })
                            } else {
                                Err(MacAddressError::IncorrectFormat)
                            }
                        }
                        ValidatorOption::NotAllow => {
                            if MAC_ADDRESS_NO_COLON_UPPERCASE_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Upper,
                                    colon: false,
                                })
                            } else {
                                Err(MacAddressError::IncorrectFormat)
                            }
                        }
                        ValidatorOption::Allow => {
                            if MAC_ADDRESS_NO_COLON_UPPERCASE_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Upper,
                                    colon: false,
                                })
                            } else if MAC_ADDRESS_NO_COLON_LOWERCASE_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Lower,
                                    colon: false,
                                })
                            } else if MAC_ADDRESS_NO_COLON_RE.is_match(mac_address) {
                                Ok(MacAddress {
                                    mac_address: String::new(),
                                    case: MacAddressCase::Both,
                                    colon: false,
                                })
                            } else {
                                Err(MacAddressError::IncorrectFormat)
                            }
                        }
                    }
                }
            }
            ValidatorOption::Allow => {
                match mac_address.len() {
                    17 => {
                        match self.lowercase {
                            ValidatorOption::Must => {
                                if MAC_ADDRESS_COLON_LOWERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Lower,
                                        colon: true,
                                    })
                                } else {
                                    Err(MacAddressError::IncorrectFormat)
                                }
                            }
                            ValidatorOption::NotAllow => {
                                if MAC_ADDRESS_COLON_UPPERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Upper,
                                        colon: true,
                                    })
                                } else {
                                    Err(MacAddressError::IncorrectFormat)
                                }
                            }
                            ValidatorOption::Allow => {
                                if MAC_ADDRESS_COLON_UPPERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Upper,
                                        colon: true,
                                    })
                                } else if MAC_ADDRESS_COLON_LOWERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Lower,
                                        colon: true,
                                    })
                                } else if MAC_ADDRESS_COLON_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Both,
                                        colon: true,
                                    })
                                } else {
                                    Err(MacAddressError::IncorrectFormat)
                                }
                            }
                        }
                    }
                    12 => {
                        match self.lowercase {
                            ValidatorOption::Must => {
                                if MAC_ADDRESS_NO_COLON_LOWERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Lower,
                                        colon: false,
                                    })
                                } else {
                                    Err(MacAddressError::IncorrectFormat)
                                }
                            }
                            ValidatorOption::NotAllow => {
                                if MAC_ADDRESS_NO_COLON_UPPERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Upper,
                                        colon: false,
                                    })
                                } else {
                                    Err(MacAddressError::IncorrectFormat)
                                }
                            }
                            ValidatorOption::Allow => {
                                if MAC_ADDRESS_NO_COLON_UPPERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Upper,
                                        colon: false,
                                    })
                                } else if MAC_ADDRESS_NO_COLON_LOWERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Lower,
                                        colon: false,
                                    })
                                } else if MAC_ADDRESS_NO_COLON_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Both,
                                        colon: false,
                                    })
                                } else {
                                    Err(MacAddressError::IncorrectFormat)
                                }
                            }
                        }
                    }
                    _ => Err(MacAddressError::IncorrectFormat),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mac_address_methods() {
        let mac_address = "08:00:27:b2:46:c3".to_string();

        let uv = MacAddressValidator {
            lowercase: ValidatorOption::Must,
            colon: ValidatorOption::Must,
        };

        let mac_address = uv.parse_string(mac_address).unwrap();

        assert_eq!("08:00:27:b2:46:c3", mac_address.get_full_mac_address());
    }

    #[test]
    fn test_mac_address_lv1() {
        let mac_address = "08:00:27:b2:46:c3".to_string();

        let bv = MacAddressValidator {
            lowercase: ValidatorOption::Allow,
            colon: ValidatorOption::Allow,
        };

        bv.parse_string(mac_address).unwrap();
    }

    #[test]
    fn test_mac_address_lv2() {
        let mac_address = "08:00:27:B2:46:C3".to_string();

        let bv = MacAddressValidator {
            lowercase: ValidatorOption::NotAllow,
            colon: ValidatorOption::Allow,
        };

        bv.parse_string(mac_address).unwrap();
    }

    #[test]
    fn test_mac_address_lv3() {
        let mac_address = "080027B246C3".to_string();

        let bv = MacAddressValidator {
            lowercase: ValidatorOption::NotAllow,
            colon: ValidatorOption::Allow,
        };

        bv.parse_string(mac_address).unwrap();
    }

    #[test]
    fn test_mac_address_lv4() {
        let mac_address = "080027B246C3".to_string();

        let bv = MacAddressValidator {
            lowercase: ValidatorOption::NotAllow,
            colon: ValidatorOption::NotAllow,
        };

        bv.parse_string(mac_address).unwrap();
    }
}

// TODO ----------

macro_rules! extend {
    ($name:ident, $lowercase:expr, $colon:expr) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(MacAddress);

        impl From<$name> for MacAddress {
            #[inline]
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0.mac_address
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = MacAddressError;

            #[inline]
            fn from_string(full_mac_address: String) -> Result<Self, Self::Error> {
                $name::from_string(full_mac_address)
            }

            #[inline]
            fn from_str(full_mac_address: &str) -> Result<Self, Self::Error> {
                $name::from_str(full_mac_address)
            }
        }

        impl Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.0))?;
                Ok(())
            }
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl $name {
            #[inline]
            pub fn from_string(full_mac_address: String) -> Result<$name, MacAddressError> {
                Ok($name($name::create_validator().parse_string(full_mac_address)?))
            }

            #[inline]
            #[allow(clippy::should_implement_trait)]
            pub fn from_str(full_mac_address: &str) -> Result<$name, MacAddressError> {
                Ok($name($name::create_validator().parse_str(full_mac_address)?))
            }

            #[inline]
            pub fn from_mac_address(mac_address: MacAddress) -> Result<$name, MacAddressError> {
                match $lowercase {
                    ValidatorOption::Must => {
                        if mac_address.has_uppercase() {
                            return Err(MacAddressError::IncorrectFormat);
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if mac_address.has_lowercase() {
                            return Err(MacAddressError::IncorrectFormat);
                        }
                    }
                    _ => (),
                }

                match $colon {
                    ValidatorOption::Must => {
                        if !mac_address.has_colon() {
                            return Err(MacAddressError::IncorrectFormat);
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if mac_address.has_colon() {
                            return Err(MacAddressError::IncorrectFormat);
                        }
                    }
                    _ => (),
                }

                Ok($name(mac_address))
            }

            #[inline]
            pub fn into_mac_address(self) -> MacAddress {
                self.0
            }

            #[inline]
            pub fn as_mac_address(&self) -> &MacAddress {
                &self.0
            }

            #[inline]
            fn create_validator() -> MacAddressValidator {
                MacAddressValidator {
                    lowercase: $lowercase,
                    colon: $colon,
                }
            }
        }

        impl $name {
            #[inline]
            pub fn get_full_mac_address(&self) -> &str {
                self.0.get_full_mac_address()
            }
        }

        impl std::str::FromStr for $name {
            type Err = MacAddressError;

            #[inline]
            fn from_str(s: &str) -> Result<$name, MacAddressError> {
                $name::from_str(s)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = MacAddressError;

            #[inline]
            fn from_form_value(
                form_value: &'a ::rocket::http::RawStr,
            ) -> Result<Self, Self::Error> {
                if $colon.allow() {
                    $name::from_string(form_value.url_decode()?)
                } else {
                    $name::from_str(form_value)
                }
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromParam<'a> for $name {
            type Error = MacAddressError;

            #[inline]
            fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                if $colon.allow() {
                    $name::from_string(param.url_decode()?)
                } else {
                    $name::from_str(param)
                }
            }
        }

        #[cfg(feature = "serdely")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            #[inline]
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: ::serde::Deserializer<'de>, {
                struct StringVisitor;

                impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
                    type Value = $name;

                    #[inline]
                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_fmt(format_args!(
                            "a mac_address({:?}) string",
                            $name::create_validator()
                        ))
                    }

                    #[inline]
                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error, {
                        $name::from_str(v).map_err(|err| E::custom(err.to_string()))
                    }

                    #[inline]
                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
                    where
                        E: ::serde::de::Error, {
                        $name::from_string(v).map_err(|err| E::custom(err.to_string()))
                    }
                }

                deserializer.deserialize_string(StringVisitor)
            }
        }

        #[cfg(feature = "serdely")]
        impl ::serde::Serialize for $name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: ::serde::Serializer, {
                serializer.serialize_str(self.0.get_full_mac_address())
            }
        }
    };
}

extend!(MacAddressAllowAnyCaseWithColon, ValidatorOption::Allow, ValidatorOption::Must);

impl MacAddressAllowAnyCaseWithColon {
    #[inline]
    pub fn has_lowercase(&self) -> bool {
        self.0.has_lowercase()
    }

    #[inline]
    pub fn has_uppercase(&self) -> bool {
        self.0.has_uppercase()
    }

    #[inline]
    pub fn has_both_case(&self) -> bool {
        self.0.has_both_case()
    }
}

extend!(MacAddressAllowAnyCaseWithoutColon, ValidatorOption::Allow, ValidatorOption::NotAllow);

impl MacAddressAllowAnyCaseWithoutColon {
    #[inline]
    pub fn has_lowercase(&self) -> bool {
        self.0.has_lowercase()
    }

    #[inline]
    pub fn has_uppercase(&self) -> bool {
        self.0.has_uppercase()
    }

    #[inline]
    pub fn has_both_case(&self) -> bool {
        self.0.has_both_case()
    }
}

extend!(MacAddressAllowAnyCaseAllowColon, ValidatorOption::Allow, ValidatorOption::Allow);

impl MacAddressAllowAnyCaseAllowColon {
    #[inline]
    pub fn has_lowercase(&self) -> bool {
        self.0.has_lowercase()
    }

    #[inline]
    pub fn has_uppercase(&self) -> bool {
        self.0.has_uppercase()
    }

    #[inline]
    pub fn has_both_case(&self) -> bool {
        self.0.has_both_case()
    }

    #[inline]
    pub fn has_colon(&self) -> bool {
        self.0.has_colon()
    }
}

extend!(MacAddressUpperCaseWithColon, ValidatorOption::NotAllow, ValidatorOption::Must);

impl MacAddressUpperCaseWithColon {}

extend!(MacAddressUpperCaseWithoutColon, ValidatorOption::NotAllow, ValidatorOption::NotAllow);

impl MacAddressUpperCaseWithoutColon {}

extend!(MacAddressUpperCaseAllowColon, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl MacAddressUpperCaseAllowColon {
    #[inline]
    pub fn has_colon(&self) -> bool {
        self.0.has_colon()
    }
}

extend!(MacAddressLowerCaseWithColon, ValidatorOption::Must, ValidatorOption::Must);

impl MacAddressLowerCaseWithColon {}

extend!(MacAddressLowerCaseWithoutColon, ValidatorOption::Must, ValidatorOption::NotAllow);

impl MacAddressLowerCaseWithoutColon {}

extend!(MacAddressLowerCaseAllowColon, ValidatorOption::Must, ValidatorOption::Allow);

impl MacAddressLowerCaseAllowColon {
    #[inline]
    pub fn has_colon(&self) -> bool {
        self.0.has_colon()
    }
}
