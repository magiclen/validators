#[macro_use]
extern crate validators;
#[cfg(feature = "rocketly")]
pub extern crate rocket;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validated_customized_string() {
        validated_customized_string!(S1, (),
            from_string input {
                Ok(input.to_string())
            },
            from_str input {
                Ok(input.to_string())
            }
        );

        validated_customized_string!(pub S2, (),
            from_string input {
                Ok(input.to_string())
            },
            from_str input {
                Ok(input.to_string())
            }
        );
    }

    #[test]
    fn test_validated_customized_regex_string() {
        validated_customized_regex_string!(S1, "^(Hi|Hello)$");
        validated_customized_regex_string!(pub S2, "^(Hi|Hello)$");
    }
}