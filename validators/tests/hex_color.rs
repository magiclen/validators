#![cfg(all(feature = "test", feature = "derive", feature = "hex_color"))]

use validators::prelude::*;

#[derive(Validator)]
#[validator(hex_color)]
struct Color(u32);

#[derive(Validator)]
#[validator(hex_color(case(Upper), alpha(Allow)))]
struct UpperColor(u32);

#[derive(Validator)]
#[validator(hex_color(case = Lower))]
struct LowerColor(u32);

#[derive(Validator)]
#[validator(hex_color(alpha(Must)))]
struct AlphaColor(u32);

#[derive(Validator)]
#[validator(hex_color(alpha = Disallow))]
struct RgbColor(u32);

#[derive(Validator)]
#[validator(hex_color(case(Lower), alpha = Must))]
struct LowerAlphaColor(u32);

#[test]
fn parse_and_format() {
    for (input, expected, formatted) in [
        ("#123", 0x112233FF, "#112233"),
        ("#1234", 0x11223344, "#11223344"),
        ("#123456", 0x123456FF, "#123456"),
        ("#12345678", 0x12345678, "#12345678"),
        ("#000", 0x000000FF, "#000000"),
        ("#FFF", 0xFFFFFFFF, "#FFFFFF"),
        ("#0123", 0x00112233, "#00112233"),
        ("#123F", 0x112233FF, "#112233"),
        ("#01020300", 0x01020300, "#01020300"),
        ("#010203FE", 0x010203FE, "#010203FE"),
    ] {
        let color = Color::parse_str(input).unwrap();

        assert_eq!(expected, color.0);
        assert_eq!(expected, Color::parse_string(String::from(input)).unwrap().0);
        assert!(Color::validate_str(input).is_ok());
        assert_eq!(formatted, color.to_hex_color_string());
        assert_eq!(expected, Color::parse_str(color.to_hex_color_string()).unwrap().0);
    }
}

#[test]
fn case() {
    assert_eq!(0xAABBCC88, Color::parse_str("#aBc8").unwrap().0);
    assert_eq!("#AABBCC88", Color::parse_str("#aBc8").unwrap().to_hex_color_string());
    assert_eq!("#AABBCC88", UpperColor::parse_str("#ABC8").unwrap().to_hex_color_string());
    assert_eq!("#aabbcc88", LowerColor::parse_str("#abc8").unwrap().to_hex_color_string());
    assert!(UpperColor::validate_str("#1234").is_ok());
    assert!(LowerColor::validate_str("#1234").is_ok());
    assert!(UpperColor::parse_str("#abc8").is_err());
    assert!(LowerColor::validate_str("#ABC8").is_err());
    assert!(UpperColor::validate_str("#123a").is_err());
    assert!(LowerColor::parse_str("#123A").is_err());
}

#[test]
fn alpha() {
    for (input, expected, formatted) in [
        ("#1234", 0x11223344, "#11223344"),
        ("#12345678", 0x12345678, "#12345678"),
        ("#123F", 0x112233FF, "#112233FF"),
        ("#123456FF", 0x123456FF, "#123456FF"),
    ] {
        let color = AlphaColor::parse_str(input).unwrap();

        assert_eq!(expected, color.0);
        assert!(AlphaColor::validate_str(input).is_ok());
        assert_eq!(expected, AlphaColor::parse_string(String::from(input)).unwrap().0);
        assert_eq!(formatted, color.to_hex_color_string());
        assert_eq!(expected, AlphaColor::parse_str(color.to_hex_color_string()).unwrap().0);
        assert!(RgbColor::parse_str(input).is_err());
    }

    for (input, expected, formatted) in
        [("#123", 0x112233FF, "#112233"), ("#123456", 0x123456FF, "#123456")]
    {
        let color = RgbColor::parse_str(input).unwrap();

        assert_eq!(expected, color.0);
        assert!(RgbColor::validate_str(input).is_ok());
        assert_eq!(expected, RgbColor::parse_string(String::from(input)).unwrap().0);
        assert_eq!(formatted, color.to_hex_color_string());
        assert_eq!(expected, RgbColor::parse_str(color.to_hex_color_string()).unwrap().0);
        assert!(AlphaColor::parse_str(input).is_err());
    }

    assert_eq!(0xAABBCCFF, LowerAlphaColor::parse_str("#abcf").unwrap().0);
    assert_eq!("#aabbccff", LowerAlphaColor::parse_str("#abcf").unwrap().to_hex_color_string());
    assert!(LowerAlphaColor::validate_str("#abcF").is_err());
}

#[test]
fn invalid_format() {
    for input in [
        "",
        "#",
        "ABC",
        "#12",
        "#12345",
        "#1234567",
        "#123456789",
        "#123G",
        " #123",
        "#123 ",
        "#123\n",
        "#éé",
    ] {
        assert!(Color::validate_str(input).is_err());
        assert!(Color::parse_str(input).is_err());
        assert!(Color::parse_string(String::from(input)).is_err());
    }
}

#[cfg(feature = "serde_json")]
#[test]
fn serde_roundtrip() {
    use validators::{Result, errors::HexColorError, serde_json};

    let color: Color = serde_json::from_str(r##""#aBc8""##).unwrap();
    let json = serde_json::to_string(&color).unwrap();
    let decoded: Color = serde_json::from_str(&json).unwrap();
    let wrapped: Result<Color, HexColorError> = serde_json::from_str(&json).unwrap();

    assert_eq!(r##""#AABBCC88""##, json);
    assert_eq!(0xAABBCC88, decoded.0);
    assert_eq!(0xAABBCC88, wrapped.into_std_result().unwrap().0);
    assert_eq!(
        r##""#AABBCC""##,
        serde_json::to_string(&UpperColor::parse_str("#ABC").unwrap()).unwrap()
    );
    assert_eq!(
        r##""#aabbcc88""##,
        serde_json::to_string(&LowerColor::parse_str("#abc8").unwrap()).unwrap()
    );
    assert_eq!(0xAABBCC88, serde_json::from_str::<LowerColor>(r##""#aabbcc88""##).unwrap().0);
    assert!(serde_json::from_str::<UpperColor>(r##""#abc8""##).is_err());

    let invalid: Result<Color, HexColorError> = serde_json::from_str(r##""#12""##).unwrap();

    assert!(invalid.into_std_result().is_err());
    assert!(serde_json::from_str::<Color>(r##""#12""##).is_err());
    assert!(serde_json::from_str::<Result<Color, HexColorError>>("123").is_err());
    assert!(serde_json::from_str::<Result<Color, HexColorError>>("\"#123").is_err());
}

#[cfg(feature = "serde_json")]
#[test]
fn serde_alpha() {
    use validators::{Result, errors::HexColorError, serde_json};

    let color: AlphaColor = serde_json::from_str(r##""#123F""##).unwrap();
    let json = serde_json::to_string(&color).unwrap();
    let decoded: AlphaColor = serde_json::from_str(&json).unwrap();
    let wrapped: Result<AlphaColor, HexColorError> = serde_json::from_str(&json).unwrap();

    assert_eq!(r##""#112233FF""##, json);
    assert_eq!(0x112233FF, decoded.0);
    assert_eq!(0x112233FF, wrapped.into_std_result().unwrap().0);

    let color: RgbColor = serde_json::from_str(r##""#123""##).unwrap();
    let json = serde_json::to_string(&color).unwrap();

    assert_eq!(r##""#112233""##, json);
    assert_eq!(0x112233FF, serde_json::from_str::<RgbColor>(&json).unwrap().0);
    assert_eq!(
        r##""#aabbccff""##,
        serde_json::to_string(&LowerAlphaColor::parse_str("#abcf").unwrap()).unwrap()
    );
    assert!(serde_json::from_str::<AlphaColor>(r##""#123""##).is_err());
    assert!(serde_json::from_str::<RgbColor>(r##""#123F""##).is_err());

    let missing: Result<AlphaColor, HexColorError> = serde_json::from_str(r##""#123""##).unwrap();
    let forbidden: Result<RgbColor, HexColorError> = serde_json::from_str(r##""#123F""##).unwrap();

    assert!(missing.into_std_result().is_err());
    assert!(forbidden.into_std_result().is_err());
}

#[cfg(feature = "serde")]
#[test]
fn deserialize_binary_string() {
    use validators::{
        Result,
        errors::HexColorError,
        serde::{self, Deserialize, de},
    };

    struct BinaryString<'a>(&'a str);

    impl<'de> de::Deserializer<'de> for BinaryString<'de> {
        type Error = de::value::Error;

        serde::forward_to_deserialize_any! {
            bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char string bytes byte_buf option unit
            unit_struct newtype_struct seq tuple tuple_struct map struct enum identifier ignored_any
        }

        fn deserialize_any<V: de::Visitor<'de>>(
            self,
            _visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            Err(de::Error::custom("deserialize_any is not supported"))
        }

        fn deserialize_str<V: de::Visitor<'de>>(
            self,
            visitor: V,
        ) -> core::result::Result<V::Value, Self::Error> {
            visitor.visit_borrowed_str(self.0)
        }

        fn is_human_readable(&self) -> bool {
            false
        }
    }

    let color = Color::deserialize(BinaryString("#AbC8")).unwrap();
    let wrapped = Result::<Color, HexColorError>::deserialize(BinaryString("#AbC8")).unwrap();

    assert_eq!(0xAABBCC88, color.0);
    assert_eq!(0xAABBCC88, wrapped.into_std_result().unwrap().0);

    let color = AlphaColor::deserialize(BinaryString("#123F")).unwrap();
    let wrapped = Result::<AlphaColor, HexColorError>::deserialize(BinaryString("#123F")).unwrap();

    assert_eq!("#112233FF", color.to_hex_color_string());
    assert_eq!(0x112233FF, wrapped.into_std_result().unwrap().0);
}

#[cfg(feature = "rocket")]
#[test]
fn rocket_fields() {
    use validators::{
        Result,
        errors::HexColorError,
        rocket::{
            form::{FromFormField, ValueField},
            request::FromParam,
        },
    };

    let color = Color::from_param("#abc").unwrap();
    let field = Color::from_value(ValueField::from_value("#abc8")).unwrap();
    let wrapped =
        Result::<Color, HexColorError>::from_value(ValueField::from_value("#abc8")).unwrap();

    assert_eq!(0xAABBCCFF, color.0);
    assert_eq!(0xAABBCC88, field.0);
    assert_eq!(0xAABBCC88, wrapped.into_std_result().unwrap().0);
}

#[cfg(feature = "axum")]
#[test]
fn axum_response() {
    use validators::{
        axum::{http::StatusCode, response::IntoResponse},
        errors::HexColorError,
    };

    assert_eq!(StatusCode::BAD_REQUEST, HexColorError.into_response().status());
}
