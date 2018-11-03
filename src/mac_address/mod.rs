extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated, ValidatedWrapper};

use std::error::Error;
use std::fmt::{self, Display, Debug, Formatter};
use std::str::Utf8Error;
use std::hash::{Hash, Hasher};

lazy_static! {
    static ref MAC_ADDRESS_COLON_UPPERCASE_RE: Regex = {
        Regex::new("^([0-9A-F]{2}:){5}[0-9A-F]{2}").unwrap()
    };
    static ref MAC_ADDRESS_COLON_LOWERCASE_RE: Regex = {
        Regex::new("^([0-9a-f]{2}:){5}[0-9a-f]{2}$").unwrap()
    };
    static ref MAC_ADDRESS_COLON_RE: Regex = {
        Regex::new("^([0-9a-fA-F]{2}:){5}[0-9a-fA-F]{2}$").unwrap()
    };
    static ref MAC_ADDRESS_NO_COLON_UPPERCASE_RE: Regex = {
        Regex::new("^[0-9A-F]{12}").unwrap()
    };
    static ref MAC_ADDRESS_NO_COLON_LOWERCASE_RE: Regex = {
        Regex::new("^[0-9a-f]{12}$").unwrap()
    };
    static ref MAC_ADDRESS_NO_COLON_RE: Regex = {
        Regex::new("^[0-9a-fA-F]{12}$").unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum MacAddressError {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for MacAddressError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for MacAddressError {}

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
    pub fn get_full_mac_address(&self) -> &str {
        &self.mac_address
    }

    pub fn has_lowercase(&self) -> bool {
        match self.case {
            MacAddressCase::Upper => {
                false
            }
            MacAddressCase::Lower => {
                true
            }
            MacAddressCase::Both => {
                true
            }
        }
    }

    pub fn has_uppercase(&self) -> bool {
        match self.case {
            MacAddressCase::Upper => {
                true
            }
            MacAddressCase::Lower => {
                false
            }
            MacAddressCase::Both => {
                true
            }
        }
    }

    pub fn has_both_case(&self) -> bool {
        match self.case {
            MacAddressCase::Both => {
                true
            }
            _ => {
                false
            }
        }
    }

    pub fn has_colon(&self) -> bool {
        self.colon
    }

    pub fn into_string(self) -> String {
        self.mac_address
    }
}

impl Validated for MacAddress {}

impl Debug for MacAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("MacAddress({})", self.mac_address))?;
        Ok(())
    }
}

impl Display for MacAddress {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.mac_address)?;
        Ok(())
    }
}

impl PartialEq for MacAddress {
    fn eq(&self, other: &Self) -> bool {
        self.mac_address.eq(&other.mac_address)
    }

    fn ne(&self, other: &Self) -> bool {
        self.mac_address.ne(&other.mac_address)
    }
}

impl Eq for MacAddress {}

impl Hash for MacAddress{
    fn hash<H: Hasher>(&self, state: &mut H){
        self.mac_address.hash(state)
    }
}

impl MacAddressValidator {
    pub fn is_mac_address(&self, mac_address: &str) -> bool {
        self.parse_inner(mac_address).is_ok()
    }

    pub fn parse_string(&self, mac_address: String) -> MacAddressResult {
        let mut mac_address_inner = self.parse_inner(&mac_address)?;

        mac_address_inner.mac_address = mac_address;

        Ok(mac_address_inner)
    }

    pub fn parse_str(&self, mac_address: &str) -> MacAddressResult {
        let mut mac_address_inner = self.parse_inner(mac_address)?;

        mac_address_inner.mac_address = mac_address.to_string();

        Ok(mac_address_inner)
    }

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
                            } else {
                                if MAC_ADDRESS_COLON_LOWERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Lower,
                                        colon: true,
                                    })
                                } else {
                                    if MAC_ADDRESS_COLON_RE.is_match(mac_address) {
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
                            } else {
                                if MAC_ADDRESS_NO_COLON_LOWERCASE_RE.is_match(mac_address) {
                                    Ok(MacAddress {
                                        mac_address: String::new(),
                                        case: MacAddressCase::Lower,
                                        colon: false,
                                    })
                                } else {
                                    if MAC_ADDRESS_NO_COLON_RE.is_match(mac_address) {
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
                                } else {
                                    if MAC_ADDRESS_COLON_LOWERCASE_RE.is_match(mac_address) {
                                        Ok(MacAddress {
                                            mac_address: String::new(),
                                            case: MacAddressCase::Lower,
                                            colon: true,
                                        })
                                    } else {
                                        if MAC_ADDRESS_COLON_RE.is_match(mac_address) {
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
                                } else {
                                    if MAC_ADDRESS_NO_COLON_LOWERCASE_RE.is_match(mac_address) {
                                        Ok(MacAddress {
                                            mac_address: String::new(),
                                            case: MacAddressCase::Lower,
                                            colon: false,
                                        })
                                    } else {
                                        if MAC_ADDRESS_NO_COLON_RE.is_match(mac_address) {
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
                    }
                    _ => {
                        Err(MacAddressError::IncorrectFormat)
                    }
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
    ( $name:ident, $lowercase:expr, $colon:expr ) => {
        #[derive(Clone)]
        pub struct $name(MacAddress);

        impl From<$name> for MacAddress {
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = MacAddressError;

            fn from_string(full_mac_address: String) -> Result<Self, Self::Error>{
                $name::from_string(full_mac_address)
            }

            fn from_str(full_mac_address: &str) -> Result<Self, Self::Error>{
                $name::from_str(full_mac_address)
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.0))?;
                Ok(())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }

            fn ne(&self, other: &Self) -> bool {
                self.0.ne(&other.0)
            }
        }

        impl PartialEq<MacAddress> for $name {
            fn eq(&self, other: &MacAddress) -> bool {
                self.0.eq(&other)
            }

            fn ne(&self, other: &MacAddress) -> bool {
                self.0.ne(&other)
            }
        }

        impl Eq for $name {}

        impl Hash for $name{
            fn hash<H: Hasher>(&self, state: &mut H){
                self.0.hash(state)
            }
        }

        impl $name {
            pub fn from_string(full_mac_address: String) -> Result<$name, MacAddressError> {
                Ok($name($name::create_validator().parse_string(full_mac_address)?))
            }

            pub fn from_str(full_mac_address: &str) -> Result<$name, MacAddressError> {
                Ok($name($name::create_validator().parse_str(full_mac_address)?))
            }

            pub fn from_mac_address(mac_address: MacAddress) -> Result<$name, MacAddressError> {
                match $lowercase {
                    ValidatorOption::Must => {
                        if mac_address.has_uppercase() {
                            return Err(MacAddressError::IncorrectFormat)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if mac_address.has_lowercase() {
                            return Err(MacAddressError::IncorrectFormat)
                        }
                    }
                    _=>()
                }

                match $colon {
                    ValidatorOption::Must => {
                        if !mac_address.has_colon() {
                            return Err(MacAddressError::IncorrectFormat)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if mac_address.has_colon() {
                            return Err(MacAddressError::IncorrectFormat)
                        }
                    }
                    _=>()
                }

                Ok($name(mac_address))
            }

            pub fn into_mac_address(self) -> MacAddress {
                self.0
            }

            pub fn as_mac_address(&self) -> &MacAddress {
                &self.0
            }

            fn create_validator() -> MacAddressValidator {
                MacAddressValidator {
                    lowercase: $lowercase,
                    colon: $colon
                }
            }
        }

        impl $name {
            pub fn get_full_mac_address(&self) -> &str {
                self.0.get_full_mac_address()
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = MacAddressError;

            fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                if $colon.allow() {
                    $name::from_string(form_value.url_decode().map_err(|err| MacAddressError::UTF8Error(err))?)
                } else {
                    $name::from_str(form_value)
                }
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromParam<'a> for $name {
            type Error = MacAddressError;

            fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                if $colon.allow() {
                    $name::from_string(param.url_decode().map_err(|err| MacAddressError::UTF8Error(err))?)
                } else {
                    $name::from_str(param)
                }
            }
        }

        #[cfg(feature = "serdely")]
        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: ::serde::Deserializer<'de> {
                struct StringVisitor;

                impl<'de> ::serde::de::Visitor<'de> for StringVisitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_fmt(format_args!("a mac_address({:?}) string", $name::create_validator()))
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        $name::from_str(v).map_err(|err| {
                            E::custom(err.to_string())
                        })
                    }

                    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: ::serde::de::Error {
                        $name::from_string(v).map_err(|err| {
                            E::custom(err.to_string())
                        })
                    }
                }

                deserializer.deserialize_string(StringVisitor)
            }
        }

        #[cfg(feature = "serdely")]
        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: ::serde::Serializer {
                serializer.serialize_str(self.0.get_full_mac_address())
            }
        }
    };
}

extend!(MacAddressAllowAnyCaseWithColon, ValidatorOption::Allow, ValidatorOption::Must);

impl MacAddressAllowAnyCaseWithColon {
    pub fn has_lowercase(&self) -> bool {
        self.0.has_lowercase()
    }

    pub fn has_uppercase(&self) -> bool {
        self.0.has_uppercase()
    }

    pub fn has_both_case(&self) -> bool {
        self.0.has_both_case()
    }
}

extend!(MacAddressAllowAnyCaseWithoutColon, ValidatorOption::Allow, ValidatorOption::NotAllow);

impl MacAddressAllowAnyCaseWithoutColon {
    pub fn has_lowercase(&self) -> bool {
        self.0.has_lowercase()
    }

    pub fn has_uppercase(&self) -> bool {
        self.0.has_uppercase()
    }

    pub fn has_both_case(&self) -> bool {
        self.0.has_both_case()
    }
}

extend!(MacAddressAllowAnyCaseAllowColon, ValidatorOption::Allow, ValidatorOption::Allow);

impl MacAddressAllowAnyCaseAllowColon {
    pub fn has_lowercase(&self) -> bool {
        self.0.has_lowercase()
    }

    pub fn has_uppercase(&self) -> bool {
        self.0.has_uppercase()
    }

    pub fn has_both_case(&self) -> bool {
        self.0.has_both_case()
    }

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
    pub fn has_colon(&self) -> bool {
        self.0.has_colon()
    }
}