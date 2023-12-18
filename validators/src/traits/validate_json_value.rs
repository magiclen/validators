use serde_json::Value;

/// Validate and deserialize `Value`s.
pub trait ValidateJsonValue: Sized {
    type Error;

    fn parse_json_value(v: Value) -> Result<Self, Self::Error>;

    fn validate_json_value(v: Value) -> Result<(), Self::Error>;
}
