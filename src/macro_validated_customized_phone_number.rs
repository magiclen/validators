#![cfg(feature = "phone-number")]

extern crate phonenumber;

#[cfg(feature = "serdely")]
use super::ValidatedWrapper;

use std::mem::transmute;
use std::error::Error;
use std::str::Utf8Error;
use std::fmt::{self, Display, Debug, Formatter};
use self::phonenumber::country::Id;

/// The region for phone numbers.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub enum PhoneNumberCountry {
    AC,
    AD,
    AE,
    AF,
    AG,
    AI,
    AL,
    AM,
    AO,
    AR,
    AS,
    AT,
    AU,
    AW,
    AX,
    AZ,
    BA,
    BB,
    BD,
    BE,
    BF,
    BG,
    BH,
    BI,
    BJ,
    BL,
    BM,
    BN,
    BO,
    BQ,
    BR,
    BS,
    BT,
    BW,
    BY,
    BZ,
    CA,
    CC,
    CD,
    CF,
    CG,
    CH,
    CI,
    CK,
    CL,
    CM,
    CN,
    CO,
    CR,
    CU,
    CV,
    CW,
    CX,
    CY,
    CZ,
    DE,
    DJ,
    DK,
    DM,
    DO,
    DZ,
    EC,
    EE,
    EG,
    EH,
    ER,
    ES,
    ET,
    FI,
    FJ,
    FK,
    FM,
    FO,
    FR,
    GA,
    GB,
    GD,
    GE,
    GF,
    GG,
    GH,
    GI,
    GL,
    GM,
    GN,
    GP,
    GQ,
    GR,
    GT,
    GU,
    GW,
    GY,
    HK,
    HN,
    HR,
    HT,
    HU,
    ID,
    IE,
    IL,
    IM,
    IN,
    IO,
    IQ,
    IR,
    IS,
    IT,
    JE,
    JM,
    JO,
    JP,
    KE,
    KG,
    KH,
    KI,
    KM,
    KN,
    KP,
    KR,
    KW,
    KY,
    KZ,
    LA,
    LB,
    LC,
    LI,
    LK,
    LR,
    LS,
    LT,
    LU,
    LV,
    LY,
    MA,
    MC,
    MD,
    ME,
    MF,
    MG,
    MH,
    MK,
    ML,
    MM,
    MN,
    MO,
    MP,
    MQ,
    MR,
    MS,
    MT,
    MU,
    MV,
    MW,
    MX,
    MY,
    MZ,
    NA,
    NC,
    NE,
    NF,
    NG,
    NI,
    NL,
    NO,
    NP,
    NR,
    NU,
    NZ,
    OM,
    PA,
    PE,
    PF,
    PG,
    PH,
    PK,
    PL,
    PM,
    PR,
    PS,
    PT,
    PW,
    PY,
    QA,
    RE,
    RO,
    RS,
    RU,
    RW,
    SA,
    SB,
    SC,
    SD,
    SE,
    SG,
    SH,
    SI,
    SJ,
    SK,
    SL,
    SM,
    SN,
    SO,
    SR,
    SS,
    ST,
    SV,
    SX,
    SY,
    SZ,
    TA,
    TC,
    TD,
    TG,
    TH,
    TJ,
    TK,
    TL,
    TM,
    TN,
    TO,
    TR,
    TT,
    TV,
    TW,
    TZ,
    UA,
    UG,
    US,
    UY,
    UZ,
    VA,
    VC,
    VE,
    VG,
    VI,
    VN,
    VU,
    WF,
    WS,
    YE,
    YT,
    ZA,
    ZM,
    ZW,
}

impl Display for PhoneNumberCountry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl PhoneNumberCountry {
    pub fn to_country_id(&self) -> Id {
        unsafe { transmute(*self) }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedPhoneNumberError {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedPhoneNumberError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for ValidatedCustomizedPhoneNumberError {}

#[cfg(feature = "serdely")]
pub struct PhoneNumberVisitor<V>(pub Vec<V>);

#[cfg(feature = "serdely")]
impl<'de, V: ValidatedWrapper> serde::de::Visitor<'de> for PhoneNumberVisitor<V> {
    type Value = V;

    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string({})", stringify!($name)))
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_str(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }

    fn visit_string<E>(self, v: String) -> Result<Self::Value, E> where E: serde::de::Error {
        V::from_string(v).map_err(|err| {
            E::custom(err.to_string())
        })
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_implement_se_de {
     ( $name:ident ) => {
        impl<'de> ::validators::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: ::validators::serde::Deserializer<'de> {
                deserializer.deserialize_string(::validators::StringVisitor(Vec::<$name>::new()))
            }
        }

        impl ::validators::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: ::validators::serde::Serializer {
                serializer.serialize_str(self.get_full_phone_number())
            }
        }
     }
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_implement_se_de {
    ( $name:ident ) => {

    }
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_implement_from_form_value {
    ( $name:ident ) => {
        impl<'a> ::validators::rocket::request::FromFormValue<'a> for $name {
            type Error = ::validators::ValidatedCustomizedPhoneNumberError;

            fn from_form_value(form_value: &'a ::validators::rocket::http::RawStr) -> std::result::Result<Self, Self::Error> {
                $name::from_string(form_value.url_decode().map_err(|err| ::validators::ValidatedCustomizedPhoneNumberError::UTF8Error(err))?)
            }
        }

        impl<'a> ::validators::rocket::request::FromParam<'a> for $name {
            type Error = ::validators::ValidatedCustomizedPhoneNumberError;

            fn from_param(param: &'a ::validators::rocket::http::RawStr) -> std::result::Result<Self, Self::Error> {
                $name::from_string(param.url_decode().map_err(|err| ::validators::ValidatedCustomizedPhoneNumberError::UTF8Error(err))?)
            }
        }
    }
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_implement_from_form_value {
    ( $name:ident ) => {

    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct {
    ( $name:ident, $field_phone_number:ident, $countries:ident, $($regions:expr), + $(,)* ) => {
        impl ::std::fmt::Debug for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_fmt(format_args!("{}({}, {:?})", stringify!($name), self.$field_phone_number, self.$countries))?;
                Ok(())
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(&self.$field_phone_number)?;
                Ok(())
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = str;

            fn deref(&self) -> &Self::Target {
                &self.$field_phone_number
            }
        }

        impl ::validators::Validated for $name {}

        impl ::validators::ValidatedWrapper for $name {
            type Error = ::validators::ValidatedCustomizedPhoneNumberError;

            fn from_string($field_phone_number: String) -> std::result::Result<Self, Self::Error> {
                $name::from_string($field_phone_number)
            }

            fn from_str($field_phone_number: &str) -> std::result::Result<Self, Self::Error> {
                $name::from_str($field_phone_number)
            }
        }

        impl<'a> $name {
            pub fn get_countries(&self) -> &[::validators::PhoneNumberCountry] {
                &self.$countries
            }

            pub fn get_full_phone_number(&self) -> &str {
                &self.$field_phone_number
            }

            pub fn into_string(self) -> String {
                self.$field_phone_number
            }

            pub fn from_string($field_phone_number: String) -> std::result::Result<Self, ::validators::ValidatedCustomizedPhoneNumberError> {
                let mut phone_number_inner = $name::from_inner(&$field_phone_number)?;

                phone_number_inner.$field_phone_number = $field_phone_number;

                Ok(phone_number_inner)
            }

            pub fn from_str($field_phone_number: &str) -> std::result::Result<Self, ::validators::ValidatedCustomizedPhoneNumberError> {
                let mut phone_number_inner = $name::from_inner($field_phone_number)?;

                phone_number_inner.$field_phone_number.push_str($field_phone_number);

                Ok(phone_number_inner)
            }

            fn from_inner(full_phone_number: &str) -> std::result::Result<Self, ::validators::ValidatedCustomizedPhoneNumberError> {
                let mut countries = Vec::new();

                $(
                    if let Ok(phone_number) = ::validators::phonenumber::parse(Some($regions.to_country_id()), full_phone_number) {
                        if phone_number.is_valid() {
                            countries.push($regions);
                        }
                    }
                )+

                if countries.is_empty() {
                    Err(::validators::ValidatedCustomizedPhoneNumberError::IncorrectFormat)
                } else {
                    Ok($name {
                        full_phone_number: String::new(),
                        countries: countries,
                    })
                }
            }

            pub unsafe fn from_string_countries_unchecked($field_phone_number: String, countries: Vec<PhoneNumberCountry>) -> Self {
                $name{$field_phone_number: $field_phone_number, $countries: countries}
            }
        }

        validated_customized_phone_number_struct_implement_from_form_value!($name);

        validated_customized_phone_number_struct_implement_se_de!($name);
    };
}

#[macro_export]
macro_rules! validated_customized_phone_number {
    ( $name:ident, $($regions:expr), + $(,)* ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct $name{
            full_phone_number: String,
            countries: Vec<::validators::PhoneNumberCountry>,
        }

        validated_customized_phone_number_struct!($name, full_phone_number, countries, $($regions, )+);
    };
    ( pub $name:ident, $($regions:expr), + $(,)* ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        pub struct $name{
            full_phone_number: String,
            countries: Vec<::validators::PhoneNumberCountry>,
        }

        validated_customized_phone_number_struct!($name, full_phone_number, countries, $($regions, )+);
    };
}
