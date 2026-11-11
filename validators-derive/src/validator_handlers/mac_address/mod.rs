mod mac_address_attribute;

use mac_address_attribute::MacAddressAttribute;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{case_option::CaseOption, separator_option::SeparatorOption, type_enum::TypeEnum},
    panic,
};

pub(crate) struct MacAddressHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::U64);

impl ValidatorHandler for MacAddressHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");
        #[cfg(feature = "serde")]
        let visitor = ident_generator.next("__ValidatorsVisitor");

        let type_attribute = MacAddressAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let mut token_stream = proc_macro2::TokenStream::new();

            let name = ast.ident;

            let error_path: Path =
                syn::parse2(quote! { validators_prelude::MacAddressError }).unwrap();

            #[cfg(feature = "test")]
            {
                let v_case = type_attribute.case;
                let v_separator = type_attribute.separator;

                token_stream.extend(quote! {
                            impl #name {
                                pub(crate) const V_CASE: validators_prelude::CaseOption = #v_case;
                                pub(crate) const V_SEPARATOR: validators_prelude::SeparatorOption = #v_separator;
                            }
                        });
            }

            let handle_iter = {
                match type_attribute.separator {
                    SeparatorOption::Allow(separator) => {
                        quote! {
                            // Separators must be used between all the groups or not at all.
                            let (first, second, third, forth, fifth, sixth) = match length {
                                12 => (&bytes[..2], &bytes[2..4], &bytes[4..6], &bytes[6..8], &bytes[8..10], &bytes[10..]),
                                17 => {
                                    if bytes[2] != #separator
                                        || bytes[5] != #separator
                                        || bytes[8] != #separator
                                        || bytes[11] != #separator
                                        || bytes[14] != #separator
                                    {
                                        return Err(#error_path::Invalid);
                                    }

                                    (&bytes[0..2], &bytes[3..5], &bytes[6..8], &bytes[9..11], &bytes[12..14], &bytes[15..])
                                },
                                _ => return Err(#error_path::Invalid),
                            };

                            first.iter().chain(second).chain(third).chain(forth).chain(fifth).chain(sixth).copied()
                        }
                    },
                    SeparatorOption::Must(separator) => {
                        quote! {
                            if length != 17 {
                                // Only the length without separators means that separators are missing.
                                return Err(if length == 12 { #error_path::SeparatorMust } else { #error_path::Invalid });
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
                    },
                    SeparatorOption::Disallow => {
                        quote! {
                            if length != 12 {
                                // Only the length with separators means that separators exist.
                                return Err(if length == 17 { #error_path::SeparatorDisallow } else { #error_path::Invalid });
                            }

                            bytes.iter().copied()
                        }
                    },
                }
            };

            let handle_decode = {
                match type_attribute.case {
                    CaseOption::Any => {
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
                    },
                    CaseOption::Upper => {
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
                    },
                    CaseOption::Lower => {
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
                    },
                }
            };

            let handle_check = {
                match type_attribute.case {
                    CaseOption::Any => {
                        quote! {
                            for e in iter {
                                match e {
                                    b'0'..=b'9' | b'a'..=b'f' | b'A'..=b'F' => (),
                                    _ => return Err(#error_path::Invalid),
                                }
                            }
                        }
                    },
                    CaseOption::Upper => {
                        quote! {
                            for e in iter {
                                match e {
                                    b'0'..=b'9' | b'A'..=b'F' => (),
                                    _ => return Err(#error_path::Invalid),
                                }
                            }
                        }
                    },
                    CaseOption::Lower => {
                        quote! {
                            for e in iter {
                                match e {
                                    b'0'..=b'9' | b'a'..=b'f' => (),
                                    _ => return Err(#error_path::Invalid),
                                }
                            }
                        }
                    },
                }
            };

            token_stream.extend(quote! {
                impl #name {
                    fn v_parse_str(s: &str) -> ::core::result::Result<u64, #error_path> {
                        let bytes = s.as_bytes();
                        let length = bytes.len();

                        let iter = {
                            #handle_iter
                        };

                        let mut mac_address_decoded = 0u64;

                        #handle_decode

                        Ok(mac_address_decoded)
                    }

                    fn v_validate_str(s: &str) -> ::core::result::Result<(), #error_path> {
                        let bytes = s.as_bytes();
                        let length = bytes.len();

                        let iter = {
                            #handle_iter
                        };

                        #handle_check

                        Ok(())
                    }
                }
            });

            token_stream.extend(quote! {
                        impl ValidateString for #name {
                            type Error = #error_path;

                            #[inline]
                            fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_str(s.into().as_str())?))
                            }

                            #[inline]
                            fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                                Ok(Self(Self::v_parse_str(s.as_ref())?))
                            }

                            #[inline]
                            fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                                Self::v_validate_str(s.as_ref())?;

                                Ok(())
                            }
                        }
                    });

            let separator = type_attribute.separator.allow().map(char::from);
            let (prepare_format, format_parameters, format_string) = if let Some(separator) =
                separator
            {
                let format_string = if type_attribute.case.upper() {
                    "{:02X}{separator}{:02X}{separator}{:02X}{separator}{:02X}{separator}{:\
                     02X}{separator}{:02X}"
                } else {
                    "{:02x}{separator}{:02x}{separator}{:02x}{separator}{:02x}{separator}{:\
                     02x}{separator}{:02x}"
                };

                (
                    quote! { let bytes: [u8; 8] = self.0.to_le_bytes(); },
                    quote! { bytes[5], bytes[4], bytes[3], bytes[2], bytes[1], bytes[0], separator = #separator },
                    format_string,
                )
            } else {
                let format_string = if type_attribute.case.upper() { "{:012X}" } else { "{:012x}" };

                (quote! {}, quote! { self.0 }, format_string)
            };

            token_stream.extend(quote! {
                impl ToMacAddressString for #name {
                    #[inline]
                    fn to_mac_address_string(&self) -> validators_prelude::String {
                        #prepare_format

                        validators_prelude::format!(#format_string, #format_parameters)
                    }
                }
            });

            #[cfg(feature = "serde")]
            {
                if type_attribute.serde_options.serialize {
                    token_stream.extend(quote! {
                                impl validators_prelude::serde::Serialize for #name {
                                    #[inline]
                                    fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                                        where
                                            S: validators_prelude::serde::Serializer, {
                                        #prepare_format

                                        serializer.collect_str(&::core::format_args!(#format_string, #format_parameters))
                                    }
                                }
                            });
                }

                if type_attribute.serde_options.deserialize {
                    use std::fmt::Write;

                    let expect = {
                        let mut s = String::from("a valid ");

                        match type_attribute.case {
                            CaseOption::Any => (),
                            CaseOption::Upper => {
                                s.push_str("upper-case ");
                            },
                            CaseOption::Lower => {
                                s.push_str("lower-case ");
                            },
                        }

                        s.push_str("MacAddress string");

                        match type_attribute.separator {
                            SeparatorOption::Must(e) => {
                                s.write_fmt(format_args!(" with separators {:?}", e as char))
                                    .unwrap();
                            },
                            SeparatorOption::Allow(e) => {
                                s.write_fmt(format_args!(
                                    " with optional separators {:?}",
                                    e as char
                                ))
                                .unwrap();
                            },
                            SeparatorOption::Disallow => s.push_str(" without separators"),
                        }

                        s
                    };

                    token_stream.extend(quote! {
                                impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                                    #[inline]
                                    fn deserialize<#deserializer>(#deserializer_name: #deserializer) -> ::core::result::Result<Self, #deserializer::Error>
                                    where
                                        #deserializer: validators_prelude::serde::Deserializer<'de>, {
                                        struct #visitor;

                                        impl<'de> validators_prelude::serde::de::Visitor<'de> for #visitor {
                                            type Value = #name;

                                            #[inline]
                                            fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                                f.write_str(#expect)
                                            }

                                            #[inline]
                                            fn visit_str<E>(self, v: &str) -> ::core::result::Result<Self::Value, E>
                                            where
                                                E: validators_prelude::serde::de::Error, {
                                                <Self::Value as ValidateString>::parse_str(v).map_err(validators_prelude::serde::de::Error::custom)
                                            }
                                        }

                                        #deserializer_name.deserialize_str(#visitor)
                                    }
                                }
                            });
                }
            }

            #[cfg(feature = "rocket")]
            {
                if type_attribute.rocket_options.from_form_field {
                    crate::common::rocket::impl_from_form_field(&mut token_stream, &name);
                }

                if type_attribute.rocket_options.from_param {
                    crate::common::rocket::impl_from_param(&mut token_stream, &name, &error_path);
                }
            }

            return Ok(token_stream);
        }

        Err(panic::validator_for_specific_item(meta.path(), ITEM))
    }
}
