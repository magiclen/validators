use quote::quote;
use syn::{Data, DeriveInput, Fields, Meta, Path};

use super::ValidatorHandler;
use crate::{
    common::{attributes::hex_attribute::HexAttribute, tri_allow::TriAllow, type_enum::TypeEnum},
    panic,
};

pub(crate) struct HexColorHandler;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Struct(TypeEnum);

const ITEM: Struct = Struct(TypeEnum::U32);

impl ValidatorHandler for HexColorHandler {
    fn meta_handler(ast: DeriveInput, meta: Meta) -> syn::Result<proc_macro2::TokenStream> {
        #[cfg(feature = "serde")]
        let mut ident_generator = crate::common::ident_generator::IdentGenerator::new(&ast);
        #[cfg(feature = "serde")]
        let deserializer = ident_generator.next("__ValidatorsDeserializer");
        #[cfg(feature = "serde")]
        let deserializer_name = ident_generator.next("__validators_deserializer");

        let type_attribute = HexAttribute::build_from_meta(&meta)?;

        if let Data::Struct(data) = ast.data
            && let Fields::Unnamed(_) = &data.fields
            && data.fields.len() == 1
        {
            let name = ast.ident;
            let error_path: Path =
                syn::parse2(quote! { validators_prelude::HexColorError }).unwrap();
            let digit_decoder = type_attribute.digit_decoder(&error_path);
            let (rgb_format, rgba_format) = if type_attribute.case.upper() {
                ("#{:06X}", "#{:08X}")
            } else {
                ("#{:06x}", "#{:08x}")
            };
            let (short_lengths, long_lengths) = match type_attribute.alpha {
                TriAllow::Must => (quote! { 4 }, quote! { 8 }),
                TriAllow::Allow => (quote! { 3 | 4 }, quote! { 6 | 8 }),
                TriAllow::Disallow => (quote! { 3 }, quote! { 6 }),
            };
            let format_color = |serialize: bool| {
                let format_value = |format, value| {
                    if serialize {
                        quote! { serializer.collect_str(&::core::format_args!(#format, #value)) }
                    } else {
                        quote! { validators_prelude::format!(#format, #value) }
                    }
                };
                let rgb = format_value(rgb_format, quote! { self.0 >> 8 });
                let rgba = format_value(rgba_format, quote! { self.0 });

                match type_attribute.alpha {
                    TriAllow::Must => rgba,
                    TriAllow::Allow => quote! {
                        if (self.0 & 0xFF) == 0xFF {
                            #rgb
                        } else {
                            #rgba
                        }
                    },
                    TriAllow::Disallow => rgb,
                }
            };
            let format_color_string = format_color(false);
            let mut token_stream = proc_macro2::TokenStream::new();

            token_stream.extend(quote! {
                impl #name {
                    #digit_decoder

                    fn v_parse_str(s: &str) -> ::core::result::Result<u32, #error_path> {
                        let digits = s.strip_prefix('#').ok_or(#error_path)?;
                        let length = digits.len();
                        let short = match length {
                            #short_lengths => true,
                            #long_lengths => false,
                            _ => return Err(#error_path),
                        };
                        let mut color = 0u32;

                        for &digit in digits.as_bytes() {
                            let digit = u32::from(Self::v_decode_hex_digit(digit)?);

                            if short {
                                color = (color << 8) | (digit * 0x11);
                            } else {
                                color = (color << 4) | digit;
                            }
                        }

                        if ::core::matches!(length, 3 | 6) {
                            color = (color << 8) | 0xFF;
                        }

                        Ok(color)
                    }
                }

                impl ValidateString for #name {
                    type Error = #error_path;

                    #[inline]
                    fn parse_string<S: Into<validators_prelude::String>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        Self::v_parse_str(s.into().as_str()).map(Self)
                    }

                    #[inline]
                    fn parse_str<S: AsRef<str>>(s: S) -> ::core::result::Result<Self, Self::Error> {
                        Self::v_parse_str(s.as_ref()).map(Self)
                    }

                    #[inline]
                    fn validate_str<S: AsRef<str>>(s: S) -> ::core::result::Result<(), Self::Error> {
                        Self::v_parse_str(s.as_ref()).map(|_| ())
                    }
                }

                impl ToHexColorString for #name {
                    #[inline]
                    fn to_hex_color_string(&self) -> validators_prelude::String {
                        #format_color_string
                    }
                }
            });

            #[cfg(feature = "serde")]
            {
                if type_attribute.serde_options.serialize {
                    let serialize_color = format_color(true);
                    token_stream.extend(quote! {
                        impl validators_prelude::serde::Serialize for #name {
                            #[inline]
                            fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
                            where
                                S: validators_prelude::serde::Serializer,
                            {
                                #serialize_color
                            }
                        }
                    });
                }

                if type_attribute.serde_options.deserialize {
                    use crate::common::case_option::CaseOption;

                    let expected = match type_attribute.case {
                        CaseOption::Any => "a CSS hexadecimal color string",
                        CaseOption::Upper => "an uppercase CSS hexadecimal color string",
                        CaseOption::Lower => "a lowercase CSS hexadecimal color string",
                    };

                    token_stream.extend(quote! {
                        impl<'de> validators_prelude::serde::Deserialize<'de> for #name {
                            #[inline]
                            fn deserialize<#deserializer>(#deserializer_name: #deserializer) -> ::core::result::Result<Self, #deserializer::Error>
                            where
                                #deserializer: validators_prelude::serde::Deserializer<'de>,
                            {
                                struct MyVisitor<T>(::core::marker::PhantomData<T>);

                                impl<'de, T: ValidateString<Error = #error_path>>
                                    validators_prelude::serde::de::Visitor<'de> for MyVisitor<T>
                                {
                                    type Value = T;

                                    fn expecting(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                                        f.write_str(#expected)
                                    }

                                    #[inline]
                                    fn visit_str<E>(self, v: &str) -> ::core::result::Result<Self::Value, E>
                                    where
                                        E: validators_prelude::serde::de::Error,
                                    {
                                        <Self::Value as ValidateString>::parse_str(v)
                                            .map_err(validators_prelude::serde::de::Error::custom)
                                    }
                                }

                                #deserializer_name.deserialize_str(MyVisitor::<Self>(::core::marker::PhantomData))
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
