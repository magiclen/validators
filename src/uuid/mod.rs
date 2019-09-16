extern crate regex;

use self::regex::Regex;
use super::{Validated, ValidatedWrapper, ValidatorOption};

use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Deref;
use std::str::Utf8Error;

lazy_static! {
    static ref UUID_UPPERCASE_RE: Regex =
        { Regex::new("^[0-9A-F]{8}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{4}-[0-9A-F]{12}$").unwrap() };
    static ref UUID_LOWERCASE_RE: Regex =
        { Regex::new("^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$").unwrap() };
    static ref UUID_RE: Regex = {
        Regex::new("^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$")
            .unwrap()
    };
}

#[derive(Debug, PartialEq, Clone)]
pub enum UUIDError {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for UUIDError {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for UUIDError {}

impl From<Utf8Error> for UUIDError {
    #[inline]
    fn from(err: Utf8Error) -> Self {
        UUIDError::UTF8Error(err)
    }
}

pub type UUIDResult = Result<UUID, UUIDError>;

#[derive(Debug, PartialEq)]
pub struct UUIDValidator {
    pub lowercase: ValidatorOption,
}

#[derive(Clone)]
enum UUIDCase {
    Upper,
    Lower,
    Both,
}

#[derive(Clone)]
pub struct UUID {
    uuid: String,
    case: UUIDCase,
}

impl UUID {
    #[inline]
    pub fn get_full_uuid(&self) -> &str {
        &self.uuid
    }

    #[inline]
    pub fn has_lowercase(&self) -> bool {
        match self.case {
            UUIDCase::Upper => false,
            UUIDCase::Lower => true,
            UUIDCase::Both => true,
        }
    }

    #[inline]
    pub fn has_uppercase(&self) -> bool {
        match self.case {
            UUIDCase::Upper => true,
            UUIDCase::Lower => false,
            UUIDCase::Both => true,
        }
    }

    #[inline]
    pub fn has_both_case(&self) -> bool {
        match self.case {
            UUIDCase::Both => true,
            _ => false,
        }
    }

    #[inline]
    pub fn into_string(self) -> String {
        self.uuid
    }
}

impl Deref for UUID {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.uuid
    }
}

impl Validated for UUID {}

impl Debug for UUID {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        impl_debug_for_tuple_struct!(UUID, f, self, let .0 = self.uuid);
    }
}

impl Display for UUID {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.uuid)?;
        Ok(())
    }
}

impl PartialEq for UUID {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.uuid.eq(&other.uuid)
    }
}

impl Eq for UUID {}

impl Hash for UUID {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.uuid.hash(state)
    }
}

impl UUIDValidator {
    #[inline]
    pub fn is_uuid(&self, uuid: &str) -> bool {
        self.parse_inner(uuid).is_ok()
    }

    #[inline]
    pub fn parse_string(&self, uuid: String) -> UUIDResult {
        let mut uuid_inner = self.parse_inner(&uuid)?;

        uuid_inner.uuid = uuid;

        Ok(uuid_inner)
    }

    #[inline]
    pub fn parse_str(&self, uuid: &str) -> UUIDResult {
        let mut uuid_inner = self.parse_inner(uuid)?;

        uuid_inner.uuid.push_str(uuid);

        Ok(uuid_inner)
    }

    fn parse_inner(&self, uuid: &str) -> UUIDResult {
        if uuid.len() != 36 {
            Err(UUIDError::IncorrectFormat)
        } else {
            match self.lowercase {
                ValidatorOption::Must => {
                    if UUID_LOWERCASE_RE.is_match(uuid) {
                        Ok(UUID {
                            uuid: String::new(),
                            case: UUIDCase::Lower,
                        })
                    } else {
                        Err(UUIDError::IncorrectFormat)
                    }
                }
                ValidatorOption::NotAllow => {
                    if UUID_UPPERCASE_RE.is_match(uuid) {
                        Ok(UUID {
                            uuid: String::new(),
                            case: UUIDCase::Upper,
                        })
                    } else {
                        Err(UUIDError::IncorrectFormat)
                    }
                }
                ValidatorOption::Allow => {
                    if UUID_UPPERCASE_RE.is_match(uuid) {
                        Ok(UUID {
                            uuid: String::new(),
                            case: UUIDCase::Upper,
                        })
                    } else if UUID_LOWERCASE_RE.is_match(uuid) {
                        Ok(UUID {
                            uuid: String::new(),
                            case: UUIDCase::Lower,
                        })
                    } else if UUID_RE.is_match(uuid) {
                        Ok(UUID {
                            uuid: String::new(),
                            case: UUIDCase::Both,
                        })
                    } else {
                        Err(UUIDError::IncorrectFormat)
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
    fn test_uuid_methods() {
        let uuid = "80a6572b-ebb8-4bf8-94b8-5c198299d118".to_string();

        let uv = UUIDValidator {
            lowercase: ValidatorOption::Must,
        };

        let uuid = uv.parse_string(uuid).unwrap();

        assert_eq!("80a6572b-ebb8-4bf8-94b8-5c198299d118", uuid.get_full_uuid());
    }

    #[test]
    fn test_uuid_lv1() {
        let uuid = "80a6572b-ebb8-4bf8-94b8-5c198299d118".to_string();

        let bv = UUIDValidator {
            lowercase: ValidatorOption::Allow,
        };

        bv.parse_string(uuid).unwrap();
    }

    #[test]
    fn test_uuid_lv2() {
        let uuid = "80A6572B-EBB8-4BF8-94B8-5C198299D118".to_string();

        let bv = UUIDValidator {
            lowercase: ValidatorOption::NotAllow,
        };

        bv.parse_string(uuid).unwrap();
    }
}

// TODO ----------

macro_rules! extend {
    ($name:ident, $lowercase:expr) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name(UUID);

        impl From<$name> for UUID {
            #[inline]
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.0.uuid
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = UUIDError;

            #[inline]
            fn from_string(full_uuid: String) -> Result<Self, Self::Error> {
                $name::from_string(full_uuid)
            }

            #[inline]
            fn from_str(full_uuid: &str) -> Result<Self, Self::Error> {
                $name::from_str(full_uuid)
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
            pub fn from_string(full_uuid: String) -> Result<$name, UUIDError> {
                Ok($name($name::create_validator().parse_string(full_uuid)?))
            }

            #[inline]
            #[allow(clippy::should_implement_trait)]
            pub fn from_str(full_uuid: &str) -> Result<$name, UUIDError> {
                Ok($name($name::create_validator().parse_str(full_uuid)?))
            }

            #[inline]
            pub fn from_uuid(uuid: UUID) -> Result<$name, UUIDError> {
                match $lowercase {
                    ValidatorOption::Must => {
                        if uuid.has_uppercase() {
                            return Err(UUIDError::IncorrectFormat);
                        }
                    }
                    ValidatorOption::NotAllow => {
                        if uuid.has_lowercase() {
                            return Err(UUIDError::IncorrectFormat);
                        }
                    }
                    _ => (),
                }

                Ok($name(uuid))
            }

            #[inline]
            pub fn into_uuid(self) -> UUID {
                self.0
            }

            #[inline]
            pub fn as_uuid(&self) -> &UUID {
                &self.0
            }

            #[inline]
            fn create_validator() -> UUIDValidator {
                UUIDValidator {
                    lowercase: $lowercase,
                }
            }
        }

        impl $name {
            #[inline]
            pub fn get_full_uuid(&self) -> &str {
                self.0.get_full_uuid()
            }
        }

        impl std::str::FromStr for $name {
            type Err = UUIDError;

            #[inline]
            fn from_str(s: &str) -> Result<$name, UUIDError> {
                $name::from_str(s)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = UUIDError;

            #[inline]
            fn from_form_value(
                form_value: &'a ::rocket::http::RawStr,
            ) -> Result<Self, Self::Error> {
                $name::from_string(form_value.url_decode()?)
            }
        }

        #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromParam<'a> for $name {
            type Error = UUIDError;

            #[inline]
            fn from_param(param: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error> {
                $name::from_string(param.url_decode()?)
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
                            "a uuid({:?}) string",
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
                serializer.serialize_str(self.0.get_full_uuid())
            }
        }
    };
}

extend!(UUIDAllowAnyCase, ValidatorOption::Allow);

impl UUIDAllowAnyCase {
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

extend!(UUIDUpperCase, ValidatorOption::NotAllow);

impl UUIDUpperCase {}

extend!(UUIDLowerCase, ValidatorOption::Must);

impl UUIDLowerCase {}
