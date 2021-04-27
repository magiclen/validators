use core::fmt::Write;

use alloc::boxed::Box;
use alloc::string::{String, ToString};

use crate::proc_macro::TokenStream;
use crate::quote::ToTokens;
use crate::syn::{Data, DeriveInput, Fields, Meta, NestedMeta, Path};

use crate::{panic, SynOption, TypeEnum, Validator, ValidatorCaseOption, ValidatorSeparatorOption};

#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::U64);
const VALIDATOR: Validator = Validator::mac_address;

pub fn mac_address_handler(ast: DeriveInput, meta: Meta) -> TokenStream {
    match ast.data {
        Data::Struct(data) => {
            if let Fields::Unnamed(_) = &data.fields {
                if data.fields.len() != 1 {
                    panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM));
                }

                let mut case = ValidatorCaseOption::new();
                let mut separator = ValidatorSeparatorOption::Allow(b':');

                let correct_usage_for_attribute = [stringify!(#[validator(mac_address)])];

                let correct_usage_for_case = [
                    stringify!(#[validator(mac_address(case(Any)))]),
                    stringify!(#[validator(mac_address(case(Upper)))]),
                    stringify!(#[validator(mac_address(case(Lower)))]),
                ];

                let correct_usage_for_separator = [
                    stringify!(#[validator(mac_address(separator(Must(colon))))]),
                    stringify!(#[validator(mac_address(separator(Must(hyphen))))]),
                    stringify!(#[validator(mac_address(separator(Allow(colon))))]),
                    stringify!(#[validator(mac_address(separator(Allow(hyphen))))]),
                    stringify!(#[validator(mac_address(separator(NotAllow)))]),
                ];

                match meta {
                    Meta::Path(_) => (),
                    Meta::List(list) => {
                        let mut case_is_set = false;
                        let mut separator_is_set = false;

                        for p in list.nested.iter() {
                            match p {
                                NestedMeta::Meta(meta) => {
                                    let meta_name = meta.path().into_token_stream().to_string();

                                    match meta_name.as_str() {
                                        "case" => {
                                            case = ValidatorCaseOption::from_meta(
                                                meta_name.as_str(),
                                                meta,
                                                &mut case_is_set,
                                                &correct_usage_for_case,
                                            );
                                        }
                                        "separator" => {
                                            separator = ValidatorSeparatorOption::from_meta(
                                                meta_name.as_str(),
                                                meta,
                                                &mut separator_is_set,
                                                &correct_usage_for_separator,
                                            );
                                        }
                                        _ => {
                                            panic::unknown_parameter(
                                                "mac_address",
                                                meta_name.as_str(),
                                            )
                                        }
                                    }
                                }
                                NestedMeta::Lit(_) => {
                                    panic::attribute_incorrect_format(
                                        "mac_address",
                                        &correct_usage_for_attribute,
                                    )
                                }
                            }
                        }
                    }
                    Meta::NameValue(_) => {
                        panic::attribute_incorrect_format(
                            "mac_address",
                            &correct_usage_for_attribute,
                        )
                    }
                }

                let name = ast.ident;

                // TODO impl

                let error_path: Path =
                    syn::parse2(quote! { validators_prelude::MacAddressError }).unwrap();

                let case_path = case.to_expr();
                let separator_expr = separator.to_expr();

                let parameters_impl = quote! {
                    impl #name {
                        pub(crate) const V_CASE: validators_prelude::ValidatorCaseOption = #case_path;
                        pub(crate) const V_SEPARATOR: validators_prelude::ValidatorSeparatorOption = #separator_expr;
                    }
                };

                let handle_iter = {
                    match separator {
                        ValidatorSeparatorOption::Allow(separator) => {
                            quote! {
                                if !(12..=17).contains(&length) {
                                    return Err(#error_path::Invalid);
                                }

                                let first = &bytes[0..2];

                                let mut no_colon_counter = if bytes[2] != #separator {
                                    1
                                } else {
                                    0
                                };

                                let second = &bytes[(3 - no_colon_counter)..(5 - no_colon_counter)];

                                if bytes[5 - no_colon_counter] != #separator {
                                    no_colon_counter += 1;
                                }

                                let third = &bytes[(6 - no_colon_counter)..(8 - no_colon_counter)];

                                if bytes[8 - no_colon_counter] != #separator {
                                    no_colon_counter += 1;
                                }

                                let forth = &bytes[(9 - no_colon_counter)..(11 - no_colon_counter)];

                                if bytes[11 - no_colon_counter] != #separator {
                                    no_colon_counter += 1;
                                }

                                let fifth = &bytes[(12 - no_colon_counter)..(14 - no_colon_counter)];

                                if bytes[14 - no_colon_counter] != #separator {
                                    no_colon_counter += 1;
                                }

                                let sixth = &bytes[(15 - no_colon_counter)..];

                                if sixth.len() != 2 {
                                    return Err(#error_path::Invalid);
                                }

                                first.iter().chain(second).chain(third).chain(forth).chain(fifth).chain(sixth).copied()
                            }
                        }
                        ValidatorSeparatorOption::Must(separator) => {
                            quote! {
                                if length != 17 {
                                    return Err(#error_path::SeparatorMust);
                                }

                                if bytes[2] != #separator
                                    || bytes[5] != #separator
                                    || bytes[8] != #separator
                                    || bytes[11] != #separator
                                    || bytes[14] != #separator
                                {
                                    return Err(#error_path::Invalid);
                                }

                                let first = &bytes[0..2];
                                let second = &bytes[3..5];
                                let third = &bytes[6..8];
                                let forth = &bytes[9..11];
                                let fifth = &bytes[12..14];
                                let sixth = &bytes[15..];

                                first.iter().chain(second).chain(third).chain(forth).chain(fifth).chain(sixth).copied()
                            }
                        }
                        ValidatorSeparatorOption::NotAllow => {
                            quote! {
                                if length != 12 {
                                    return Err(#error_path::SeparatorNotAllow);
                                }

                                bytes.iter().copied()
                            }
                        }
                    }
                };

                let handle_decode = {
                    match case {
                        ValidatorCaseOption::Any => {
                            quote! {
                                for e in iter {
                                    mac_address_decoded <<= 4;

                                    match e {
                                        b'0'..=b'9' => {
                                            mac_address_decoded |= u64::from(e - b'0');
                                        }
                                        b'a'..=b'f' => {
                                            mac_address_decoded |= u64::from(e - (b'a' - 10));
                                        }
                                        b'A'..=b'F' => {
                                            mac_address_decoded |= u64::from(e - (b'A' - 10));
                                        }
                                        _ => return Err(#error_path::Invalid),
                                    }
                                }
                            }
                        }
                        ValidatorCaseOption::Upper => {
                            quote! {
                                for e in iter {
                                    mac_address_decoded <<= 4;

                                    match e {
                                        b'0'..=b'9' => {
                                            mac_address_decoded |= u64::from(e - b'0');
                                        }
                                        b'A'..=b'F' => {
                                            mac_address_decoded |= u64::from(e - (b'A' - 10));
                                        }
                                        _ => return Err(#error_path::Invalid),
                                    }
                                }
                            }
                        }
                        ValidatorCaseOption::Lower => {
                            quote! {
                                for e in iter {
                                    mac_address_decoded <<= 4;

                                    match e {
                                        b'0'..=b'9' => {
                                            mac_address_decoded |= u64::from(e - b'0');
                                        }
                                        b'a'..=b'f' => {
                                            mac_address_decoded |= u64::from(e - (b'a' - 10));
                                        }
                                        _ => return Err(#error_path::Invalid),
                                    }
                                }
                            }
                        }
                    }
                };

                let handle_check = {
                    match case {
                        ValidatorCaseOption::Any => {
                            quote! {
                                for e in iter {
                                    match e {
                                        b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => (),
                                        _ => return Err(#error_path::Invalid),
                                    }
                                }
                            }
                        }
                        ValidatorCaseOption::Upper => {
                            quote! {
                                for e in iter {
                                    match e {
                                        b'0'..=b'9' | b'A'..=b'F' => (),
                                        _ => return Err(#error_path::Invalid),
                                    }
                                }
                            }
                        }
                        ValidatorCaseOption::Lower => {
                            quote! {
                                for e in iter {
                                    match e {
                                        b'0'..=b'9' | b'a'..=b'f' => (),
                                        _ => return Err(#error_path::Invalid),
                                    }
                                }
                            }
                        }
                    }
                };

                let v_parse_str = quote! {
                    #[inline]
                    fn v_parse_str(s: &str) -> Result<u64, #error_path> {
                        let bytes = s.as_bytes();
                        let length = bytes.len();

                        let iter = {
                            #handle_iter
                        };

                        let mut mac_address_decoded = 0u64;

                        #handle_decode

                        Ok(mac_address_decoded)
                    }
                };

                let v_validate_str = quote! {
                    #[inline]
                    fn v_validate_str(s: &str) -> Result<(), #error_path> {
                        let bytes = s.as_bytes();
                        let length = bytes.len();

                        let iter = {
                            #handle_iter
                        };

                        #handle_check

                        Ok(())
                    }
                };

                let parse_impl = quote! {
                    impl #name {
                        #v_parse_str

                        #v_validate_str
                    }
                };

                let to_mac_address_string = {
                    match case {
                        ValidatorCaseOption::Lower => {
                            match separator {
                                ValidatorSeparatorOption::Allow(separator)
                                | ValidatorSeparatorOption::Must(separator) => {
                                    let separator = separator as char;

                                    quote! {
                                        #[inline]
                                        pub fn to_mac_address_string(&self) -> validators_prelude::String {
                                            let bytes: [u8; 8] = self.0.to_le_bytes();

                                            validators_prelude::format!(
                                                "{:02x}{separator}{:02x}{separator}{:02x}{separator}{:02x}{separator}{:02x}{separator}{:02x}",
                                                bytes[5], bytes[4], bytes[3], bytes[2], bytes[1], bytes[0], separator = #separator
                                            )
                                        }
                                    }
                                }
                                ValidatorSeparatorOption::NotAllow => {
                                    quote! {
                                        #[inline]
                                        pub fn to_mac_address_string(&self) -> validators_prelude::String {
                                            validators_prelude::format!("{:012x}", self.0)
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            match separator {
                                ValidatorSeparatorOption::Allow(separator)
                                | ValidatorSeparatorOption::Must(separator) => {
                                    let separator = separator as char;

                                    quote! {
                                        #[inline]
                                        pub fn to_mac_address_string(&self) -> validators_prelude::String {
                                            let bytes: [u8; 8] = self.0.to_le_bytes();

                                            validators_prelude::format!(
                                                "{:02X}{separator}{:02X}{separator}{:02X}{separator}{:02X}{separator}{:02X}{separator}{:02X}",
                                                bytes[5], bytes[4], bytes[3], bytes[2], bytes[1], bytes[0], separator = #separator
                                            )
                                        }
                                    }
                                }
                                ValidatorSeparatorOption::NotAllow => {
                                    quote! {
                                        #[inline]
                                        pub fn to_mac_address_string(&self) -> validators_prelude::String {
                                            validators_prelude::format!("{:012X}", self.0)
                                        }
                                    }
                                }
                            }
                        }
                    }
                };

                let other_functions = quote! {
                    impl #name {
                        #to_mac_address_string
                    }
                };

                let validate_string_impl = quote! {
                    impl ValidateString for #name {
                        type Error = #error_path;
                        type Output = Self;

                        #[inline]
                        fn parse_string<S: Into<validators_prelude::String>>(s: S) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_str(s.into().as_str())?))
                        }

                        #[inline]
                        fn parse_str<S: AsRef<str>>(s: S) -> Result<Self::Output, Self::Error> {
                            Ok(#name(Self::v_parse_str(s.as_ref())?))
                        }

                        #[inline]
                        fn validate_str<S: AsRef<str>>(s: S) -> Result<(), Self::Error> {
                            Self::v_validate_str(s.as_ref())?;

                            Ok(())
                        }
                    }
                };

                let serde_impl = if cfg!(feature = "serde") {
                    let expect = {
                        let mut s = String::from("a valid ");

                        match case {
                            ValidatorCaseOption::Any => (),
                            ValidatorCaseOption::Upper => {
                                s.push_str("upper-case ");
                            }
                            ValidatorCaseOption::Lower => {
                                s.push_str("lower-case ");
                            }
                        }

                        s.push_str("MacAddress string");

                        match separator {
                            ValidatorSeparatorOption::Must(e) => {
                                s.write_fmt(format_args!(" with separators {:?}", e as char))
                                    .unwrap();
                            }
                            ValidatorSeparatorOption::Allow(e) => {
                                s.write_fmt(format_args!(
                                    " with optional separators {:?}",
                                    e as char
                                ))
                                .unwrap();
                            }
                            ValidatorSeparatorOption::NotAllow => s.push_str(" without separators"),
                        }

                        s
                    };

                    quote! {
                        impl validators_prelude::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                                where
                                    S: validators_prelude::Serializer, {
                                serializer.serialize_str(&self.to_mac_address_string())
                            }
                        }

                        impl<'de> validators_prelude::Deserialize<'de> for #name {
                            #[inline]
                            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                            where
                                D: validators_prelude::Deserializer<'de>, {
                                struct ValidatingVisitor;

                                impl<'de> validators_prelude::Visitor<'de> for ValidatingVisitor {
                                    type Value = #name;

                                    #[inline]
                                    fn expecting(&self, f: &mut validators_prelude::Formatter) -> Result<(), validators_prelude::fmt::Error> {
                                        f.write_str(#expect)
                                    }

                                    #[inline]
                                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateString>::parse_str(v).map_err(validators_prelude::DeError::custom)
                                    }

                                    #[inline]
                                    fn visit_string<E>(self, v: validators_prelude::String) -> Result<Self::Value, E>
                                    where
                                        E: validators_prelude::DeError, {
                                        <#name as ValidateString>::parse_string(v).map_err(validators_prelude::DeError::custom)
                                    }
                                }

                                deserializer.deserialize_string(ValidatingVisitor)
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let rocket_impl = if cfg!(feature = "rocket") {
                    quote! {
                        impl<'a> validators_prelude::FromFormField<'a> for #name {
                            #[inline]
                            fn from_value(v: validators_prelude::ValueField<'a>) -> validators_prelude::FormResult<'a, Self> {
                                Ok(<#name as ValidateString>::parse_str(v.value).map_err(validators_prelude::FormError::custom)?)
                            }
                        }

                        impl<'a> validators_prelude::FromParam<'a> for #name {
                            type Error = #error_path;

                            #[inline]
                            fn from_param(v: &'a str) -> Result<Self, Self::Error> {
                                <#name as ValidateString>::parse_str(v)
                            }
                        }
                    }
                } else {
                    quote! {}
                };

                let mac_address_impl = quote! {
                    #parameters_impl

                    #parse_impl

                    #validate_string_impl

                    #other_functions

                    #serde_impl

                    #rocket_impl
                };

                mac_address_impl.into()
            } else {
                panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM))
            }
        }
        _ => panic::validator_only_support_for_item(VALIDATOR, Box::new(ITEM)),
    }
}
