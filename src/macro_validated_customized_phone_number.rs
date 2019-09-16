#![cfg(feature = "phone-number")]

extern crate phonenumber;

#[cfg(feature = "serdely")]
use super::ValidatedWrapper;

use self::phonenumber::country::Id;
use std::error::Error;
use std::fmt::{self, Debug, Display, Formatter};
use std::mem::transmute;
use std::str::Utf8Error;

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
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl PhoneNumberCountry {
    #[inline]
    pub fn to_country_id(self) -> Id {
        unsafe { transmute(self) }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ValidatedCustomizedPhoneNumberError {
    IncorrectFormat,
    UTF8Error(Utf8Error),
}

impl Display for ValidatedCustomizedPhoneNumberError {
    #[inline]
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

    #[inline]
    fn expecting(&self, formatter: &mut Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!("a string({})", stringify!($name)))
    }

    #[inline]
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        V::from_str(v).map_err(|err| E::custom(err.to_string()))
    }

    #[inline]
    fn visit_string<E>(self, v: String) -> Result<Self::Value, E>
    where
        E: serde::de::Error, {
        V::from_string(v).map_err(|err| E::custom(err.to_string()))
    }
}

#[cfg(feature = "serdely")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_implement_se_de {
    ($name:ident) => {
        impl<'de> $crate::serde::Deserialize<'de> for $name {
            #[inline]
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: $crate::serde::Deserializer<'de>, {
                deserializer.deserialize_string($crate::StringVisitor(Vec::<$name>::new()))
            }
        }

        impl $crate::serde::Serialize for $name {
            #[inline]
            fn serialize<S>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error>
            where
                S: $crate::serde::Serializer, {
                serializer.serialize_str(self.get_full_phone_number())
            }
        }
    };
}

#[cfg(not(feature = "serdely"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_implement_se_de {
    ($name:ident) => {};
}

#[cfg(feature = "rocketly")]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_implement_from_form_value {
    ($name:ident) => {
        impl<'a> $crate::rocket::request::FromFormValue<'a> for $name {
            type Error = $crate::ValidatedCustomizedPhoneNumberError;

            #[inline]
            fn from_form_value(
                form_value: &'a $crate::rocket::http::RawStr,
            ) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(
                    form_value.url_decode().map_err(|err| {
                        $crate::ValidatedCustomizedPhoneNumberError::UTF8Error(err)
                    })?,
                )
            }
        }

        impl<'a> $crate::rocket::request::FromParam<'a> for $name {
            type Error = $crate::ValidatedCustomizedPhoneNumberError;

            #[inline]
            fn from_param(
                param: &'a $crate::rocket::http::RawStr,
            ) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string(
                    param.url_decode().map_err(|err| {
                        $crate::ValidatedCustomizedPhoneNumberError::UTF8Error(err)
                    })?,
                )
            }
        }
    };
}

#[cfg(not(feature = "rocketly"))]
#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_implement_from_form_value {
    ($name:ident) => {};
}

#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct_inner {
    ( $full_phone_number:ident, $countries:ident $(,)* ) => {
        {
            use $crate::PhoneNumberCountry;
            validated_customized_phone_number_struct_inner!($full_phone_number, $countries,
                PhoneNumberCountry::AC,
                PhoneNumberCountry::AD,
                PhoneNumberCountry::AE,
                PhoneNumberCountry::AF,
                PhoneNumberCountry::AG,
                PhoneNumberCountry::AI,
                PhoneNumberCountry::AL,
                PhoneNumberCountry::AM,
                PhoneNumberCountry::AO,
                PhoneNumberCountry::AR,
                PhoneNumberCountry::AS,
                PhoneNumberCountry::AT,
                PhoneNumberCountry::AU,
                PhoneNumberCountry::AW,
                PhoneNumberCountry::AX,
                PhoneNumberCountry::AZ,
                PhoneNumberCountry::BA,
                PhoneNumberCountry::BB,
                PhoneNumberCountry::BD,
                PhoneNumberCountry::BE,
                PhoneNumberCountry::BF,
                PhoneNumberCountry::BG,
                PhoneNumberCountry::BH,
                PhoneNumberCountry::BI,
                PhoneNumberCountry::BJ,
                PhoneNumberCountry::BL,
                PhoneNumberCountry::BM,
                PhoneNumberCountry::BN,
                PhoneNumberCountry::BO,
                PhoneNumberCountry::BQ,
                PhoneNumberCountry::BR,
                PhoneNumberCountry::BS,
                PhoneNumberCountry::BT,
                PhoneNumberCountry::BW,
                PhoneNumberCountry::BY,
                PhoneNumberCountry::BZ,
                PhoneNumberCountry::CA,
                PhoneNumberCountry::CC,
                PhoneNumberCountry::CD,
                PhoneNumberCountry::CF,
                PhoneNumberCountry::CG,
                PhoneNumberCountry::CH,
                PhoneNumberCountry::CI,
                PhoneNumberCountry::CK,
                PhoneNumberCountry::CL,
                PhoneNumberCountry::CM,
                PhoneNumberCountry::CN,
                PhoneNumberCountry::CO,
                PhoneNumberCountry::CR,
                PhoneNumberCountry::CU,
                PhoneNumberCountry::CV,
                PhoneNumberCountry::CW,
                PhoneNumberCountry::CX,
                PhoneNumberCountry::CY,
                PhoneNumberCountry::CZ,
                PhoneNumberCountry::DE,
                PhoneNumberCountry::DJ,
                PhoneNumberCountry::DK,
                PhoneNumberCountry::DM,
                PhoneNumberCountry::DO,
                PhoneNumberCountry::DZ,
                PhoneNumberCountry::EC,
                PhoneNumberCountry::EE,
                PhoneNumberCountry::EG,
                PhoneNumberCountry::EH,
                PhoneNumberCountry::ER,
                PhoneNumberCountry::ES,
                PhoneNumberCountry::ET,
                PhoneNumberCountry::FI,
                PhoneNumberCountry::FJ,
                PhoneNumberCountry::FK,
                PhoneNumberCountry::FM,
                PhoneNumberCountry::FO,
                PhoneNumberCountry::FR,
                PhoneNumberCountry::GA,
                PhoneNumberCountry::GB,
                PhoneNumberCountry::GD,
                PhoneNumberCountry::GE,
                PhoneNumberCountry::GF,
                PhoneNumberCountry::GG,
                PhoneNumberCountry::GH,
                PhoneNumberCountry::GI,
                PhoneNumberCountry::GL,
                PhoneNumberCountry::GM,
                PhoneNumberCountry::GN,
                PhoneNumberCountry::GP,
                PhoneNumberCountry::GQ,
                PhoneNumberCountry::GR,
                PhoneNumberCountry::GT,
                PhoneNumberCountry::GU,
                PhoneNumberCountry::GW,
                PhoneNumberCountry::GY,
                PhoneNumberCountry::HK,
                PhoneNumberCountry::HN,
                PhoneNumberCountry::HR,
                PhoneNumberCountry::HT,
                PhoneNumberCountry::HU,
                PhoneNumberCountry::ID,
                PhoneNumberCountry::IE,
                PhoneNumberCountry::IL,
                PhoneNumberCountry::IM,
                PhoneNumberCountry::IN,
                PhoneNumberCountry::IO,
                PhoneNumberCountry::IQ,
                PhoneNumberCountry::IR,
                PhoneNumberCountry::IS,
                PhoneNumberCountry::IT,
                PhoneNumberCountry::JE,
                PhoneNumberCountry::JM,
                PhoneNumberCountry::JO,
                PhoneNumberCountry::JP,
                PhoneNumberCountry::KE,
                PhoneNumberCountry::KG,
                PhoneNumberCountry::KH,
                PhoneNumberCountry::KI,
                PhoneNumberCountry::KM,
                PhoneNumberCountry::KN,
                PhoneNumberCountry::KP,
                PhoneNumberCountry::KR,
                PhoneNumberCountry::KW,
                PhoneNumberCountry::KY,
                PhoneNumberCountry::KZ,
                PhoneNumberCountry::LA,
                PhoneNumberCountry::LB,
                PhoneNumberCountry::LC,
                PhoneNumberCountry::LI,
                PhoneNumberCountry::LK,
                PhoneNumberCountry::LR,
                PhoneNumberCountry::LS,
                PhoneNumberCountry::LT,
                PhoneNumberCountry::LU,
                PhoneNumberCountry::LV,
                PhoneNumberCountry::LY,
                PhoneNumberCountry::MA,
                PhoneNumberCountry::MC,
                PhoneNumberCountry::MD,
                PhoneNumberCountry::ME,
                PhoneNumberCountry::MF,
                PhoneNumberCountry::MG,
                PhoneNumberCountry::MH,
                PhoneNumberCountry::MK,
                PhoneNumberCountry::ML,
                PhoneNumberCountry::MM,
                PhoneNumberCountry::MN,
                PhoneNumberCountry::MO,
                PhoneNumberCountry::MP,
                PhoneNumberCountry::MQ,
                PhoneNumberCountry::MR,
                PhoneNumberCountry::MS,
                PhoneNumberCountry::MT,
                PhoneNumberCountry::MU,
                PhoneNumberCountry::MV,
                PhoneNumberCountry::MW,
                PhoneNumberCountry::MX,
                PhoneNumberCountry::MY,
                PhoneNumberCountry::MZ,
                PhoneNumberCountry::NA,
                PhoneNumberCountry::NC,
                PhoneNumberCountry::NE,
                PhoneNumberCountry::NF,
                PhoneNumberCountry::NG,
                PhoneNumberCountry::NI,
                PhoneNumberCountry::NL,
                PhoneNumberCountry::NO,
                PhoneNumberCountry::NP,
                PhoneNumberCountry::NR,
                PhoneNumberCountry::NU,
                PhoneNumberCountry::NZ,
                PhoneNumberCountry::OM,
                PhoneNumberCountry::PA,
                PhoneNumberCountry::PE,
                PhoneNumberCountry::PF,
                PhoneNumberCountry::PG,
                PhoneNumberCountry::PH,
                PhoneNumberCountry::PK,
                PhoneNumberCountry::PL,
                PhoneNumberCountry::PM,
                PhoneNumberCountry::PR,
                PhoneNumberCountry::PS,
                PhoneNumberCountry::PT,
                PhoneNumberCountry::PW,
                PhoneNumberCountry::PY,
                PhoneNumberCountry::QA,
                PhoneNumberCountry::RE,
                PhoneNumberCountry::RO,
                PhoneNumberCountry::RS,
                PhoneNumberCountry::RU,
                PhoneNumberCountry::RW,
                PhoneNumberCountry::SA,
                PhoneNumberCountry::SB,
                PhoneNumberCountry::SC,
                PhoneNumberCountry::SD,
                PhoneNumberCountry::SE,
                PhoneNumberCountry::SG,
                PhoneNumberCountry::SH,
                PhoneNumberCountry::SI,
                PhoneNumberCountry::SJ,
                PhoneNumberCountry::SK,
                PhoneNumberCountry::SL,
                PhoneNumberCountry::SM,
                PhoneNumberCountry::SN,
                PhoneNumberCountry::SO,
                PhoneNumberCountry::SR,
                PhoneNumberCountry::SS,
                PhoneNumberCountry::ST,
                PhoneNumberCountry::SV,
                PhoneNumberCountry::SX,
                PhoneNumberCountry::SY,
                PhoneNumberCountry::SZ,
                PhoneNumberCountry::TA,
                PhoneNumberCountry::TC,
                PhoneNumberCountry::TD,
                PhoneNumberCountry::TG,
                PhoneNumberCountry::TH,
                PhoneNumberCountry::TJ,
                PhoneNumberCountry::TK,
                PhoneNumberCountry::TL,
                PhoneNumberCountry::TM,
                PhoneNumberCountry::TN,
                PhoneNumberCountry::TO,
                PhoneNumberCountry::TR,
                PhoneNumberCountry::TT,
                PhoneNumberCountry::TV,
                PhoneNumberCountry::TW,
                PhoneNumberCountry::TZ,
                PhoneNumberCountry::UA,
                PhoneNumberCountry::UG,
                PhoneNumberCountry::US,
                PhoneNumberCountry::UY,
                PhoneNumberCountry::UZ,
                PhoneNumberCountry::VA,
                PhoneNumberCountry::VC,
                PhoneNumberCountry::VE,
                PhoneNumberCountry::VG,
                PhoneNumberCountry::VI,
                PhoneNumberCountry::VN,
                PhoneNumberCountry::VU,
                PhoneNumberCountry::WF,
                PhoneNumberCountry::WS,
                PhoneNumberCountry::YE,
                PhoneNumberCountry::YT,
                PhoneNumberCountry::ZA,
                PhoneNumberCountry::ZM,
                PhoneNumberCountry::ZW,
            );
        }
    };
    ( $full_phone_number:ident, $countries:ident, $($regions:expr), + $(,)* ) => {
        $(
            if let Ok(phone_number) = $crate::phonenumber::parse(Some($regions.to_country_id()), $full_phone_number) {
                if phone_number.is_valid() {
                    $countries.push($regions);
                }
            }
        )+
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! validated_customized_phone_number_struct {
    ( $name:ident, $field_phone_number:ident, $countries:ident $(, $regions:expr) * $(,)* ) => {
        impl ::std::fmt::Debug for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                $crate::debug_helper::impl_debug_for_tuple_struct!($name, f, self, let .0 = self.$field_phone_number, let .1 = self.$countries);
            }
        }

        impl ::std::fmt::Display for $name {
            #[inline]
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                f.write_str(&self.$field_phone_number)?;
                Ok(())
            }
        }

        impl ::std::ops::Deref for $name {
            type Target = str;

            #[inline]
            fn deref(&self) -> &Self::Target {
                &self.$field_phone_number
            }
        }

        impl $crate::Validated for $name {}

        impl $crate::ValidatedWrapper for $name {
            type Error = $crate::ValidatedCustomizedPhoneNumberError;

            #[inline]
            fn from_string($field_phone_number: String) -> ::std::result::Result<Self, Self::Error> {
                $name::from_string($field_phone_number)
            }

            #[inline]
            fn from_str($field_phone_number: &str) -> ::std::result::Result<Self, Self::Error> {
                $name::from_str($field_phone_number)
            }
        }

        impl<'a> $name {
            #[inline]
            pub fn get_countries(&self) -> &[$crate::PhoneNumberCountry] {
                &self.$countries
            }

            #[inline]
            pub fn get_full_phone_number(&self) -> &str {
                &self.$field_phone_number
            }

            #[inline]
            pub fn into_string(self) -> String {
                self.$field_phone_number
            }

            #[inline]
            pub fn from_string($field_phone_number: String) -> ::std::result::Result<Self, $crate::ValidatedCustomizedPhoneNumberError> {
                let mut phone_number_inner = $name::from_inner(&$field_phone_number)?;

                phone_number_inner.$field_phone_number = $field_phone_number;

                Ok(phone_number_inner)
            }

            #[inline]
            pub fn from_str($field_phone_number: &str) -> ::std::result::Result<Self, $crate::ValidatedCustomizedPhoneNumberError> {
                let mut phone_number_inner = $name::from_inner($field_phone_number)?;

                phone_number_inner.$field_phone_number.push_str($field_phone_number);

                Ok(phone_number_inner)
            }

            #[inline]
            fn from_inner($field_phone_number: &str) -> ::std::result::Result<Self, $crate::ValidatedCustomizedPhoneNumberError> {
                let mut countries = Vec::new();

                validated_customized_phone_number_struct_inner!($field_phone_number, countries, $($regions, )*);

                if countries.is_empty() {
                    Err($crate::ValidatedCustomizedPhoneNumberError::IncorrectFormat)
                } else {
                    Ok($name {
                        $field_phone_number: String::new(),
                        $countries: countries,
                    })
                }
            }

            #[inline]
            pub unsafe fn from_string_countries_unchecked($field_phone_number: String, countries: Vec<$crate::PhoneNumberCountry>) -> Self {
                $name {$field_phone_number: $field_phone_number, $countries: countries}
            }
        }

        impl ::std::str::FromStr for $name {
            type Err = $crate::ValidatedCustomizedPhoneNumberError;

            #[inline]
            fn from_str(s: &str) -> Result<Self, $crate::ValidatedCustomizedPhoneNumberError> {
                $name::from_str(s)
            }
        }

        validated_customized_phone_number_struct_implement_from_form_value!($name);

        validated_customized_phone_number_struct_implement_se_de!($name);
    };
}

#[macro_export]
macro_rules! validated_customized_phone_number {
    ( $name:ident $(, $regions:expr ) * $(,)* ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        struct $name {
            full_phone_number: String,
            countries: Vec<$crate::PhoneNumberCountry>,
        }

        validated_customized_phone_number_struct!($name, full_phone_number, countries, $($regions, )*);
    };
    ( $v:vis $name:ident $(, $regions:expr ) * $(,)* ) => {
        #[derive(Clone, PartialEq, Eq, Hash)]
        $v struct $name {
            full_phone_number: String,
            countries: Vec<$crate::PhoneNumberCountry>,
        }

        validated_customized_phone_number_struct!($name, full_phone_number, countries, $($regions, )*);
    };
}
