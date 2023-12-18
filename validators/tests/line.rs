#![cfg(all(feature = "test", feature = "derive", feature = "line"))]

use validators::prelude::*;

fn check_char_length(
    s: &str,
    min: Option<usize>,
    max: Option<usize>,
    trimmed_min: Option<usize>,
) -> bool {
    let length = s.chars().count();
    let trimmed_length = s.trim().chars().count();

    if let Some(min) = min {
        if length < min {
            return false;
        }
    }

    if let Some(max) = max {
        if length > max {
            return false;
        }
    }

    if let Some(trimmed_min) = trimmed_min {
        if trimmed_length < trimmed_min {
            return false;
        }
    }

    true
}

fn check_byte_length(
    s: &str,
    min: Option<usize>,
    max: Option<usize>,
    trimmed_min: Option<usize>,
) -> bool {
    let length = s.len();
    let trimmed_length = s.trim().len();

    if let Some(min) = min {
        if length < min {
            return false;
        }
    }

    if let Some(max) = max {
        if length > max {
            return false;
        }
    }

    if let Some(trimmed_min) = trimmed_min {
        if trimmed_length < trimmed_min {
            return false;
        }
    }

    true
}

#[test]
fn char() {
    macro_rules! test {
        ($( { $( $p:meta => $v:expr ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(line(char_length($($p = $v,)*)))]
                    pub struct Validator(pub String);

                    fn test(s: &str, is_ok: bool) {
                        let panic = match Validator::validate_str(s) {
                            Ok(_) => !is_ok,
                            Err(_) if !is_ok => false,
                            Err(err) => {
                                eprintln!("{}", err);

                                true
                            }
                        };

                        if panic {
                            panic!("{:?}: {} expect {}", s, stringify! {
                                $(
                                    $p = $v,
                                )*
                            }, is_ok);
                        }
                    }

                    test("123\n456", false);
                    test("", check_char_length("", Validator::V_CHAR_LENGTH_MIN, Validator::V_CHAR_LENGTH_MAX, Validator::V_CHAR_LENGTH_TRIMMED_MIN));
                    test("   ", check_char_length("   ", Validator::V_CHAR_LENGTH_MIN, Validator::V_CHAR_LENGTH_MAX, Validator::V_CHAR_LENGTH_TRIMMED_MIN));
                    test("　　　", check_char_length("　　　", Validator::V_CHAR_LENGTH_MIN, Validator::V_CHAR_LENGTH_MAX, Validator::V_CHAR_LENGTH_TRIMMED_MIN));
                    test("123", check_char_length("123", Validator::V_CHAR_LENGTH_MIN, Validator::V_CHAR_LENGTH_MAX, Validator::V_CHAR_LENGTH_TRIMMED_MIN));
                }
            )*
        }
    }

    test! {
        {
            min => 1,
        },
        {
            trimmed_min => 1,
        },
        {
            max => 3,
        },
        {
            min => 1,
            max => 3,
        },
        {
            trimmed_min => 1,
            max => 3,
        },
        {
            trimmed_min => 1,
            min => 2,
            max => 3,
        },
    }
}

#[test]
fn byte() {
    macro_rules! test {
        ($( { $( $p:meta => $v:expr ),* $(,)* } ),* $(,)* ) => {
            $(
                {
                    #[derive(Validator)]
                    #[validator(line(byte_length($($p = $v,)*)))]
                    pub struct Validator(pub String);

                    fn test(s: &str, is_ok: bool) {
                        let panic = match Validator::validate_str(s) {
                            Ok(_) => !is_ok,
                            Err(_) if !is_ok => false,
                            Err(err) => {
                                eprintln!("{}", err);

                                true
                            }
                        };

                        if panic {
                            panic!("{:?}: {} expect {}", s, stringify! {
                                $(
                                    $p = $v,
                                )*
                            }, is_ok);
                        }
                    }

                    test("123\n456", false);
                    test("", check_byte_length("", Validator::V_BYTE_LENGTH_MIN, Validator::V_BYTE_LENGTH_MAX, Validator::V_BYTE_LENGTH_TRIMMED_MIN));
                    test("   ", check_byte_length("   ", Validator::V_BYTE_LENGTH_MIN, Validator::V_BYTE_LENGTH_MAX, Validator::V_BYTE_LENGTH_TRIMMED_MIN));
                    test("　　　", check_byte_length("　　　", Validator::V_BYTE_LENGTH_MIN, Validator::V_BYTE_LENGTH_MAX, Validator::V_BYTE_LENGTH_TRIMMED_MIN));
                    test("123", check_byte_length("123", Validator::V_BYTE_LENGTH_MIN, Validator::V_BYTE_LENGTH_MAX, Validator::V_BYTE_LENGTH_TRIMMED_MIN));
                }
            )*
        }
    }

    test! {
        {
            min => 1,
        },
        {
            trimmed_min => 1,
        },
        {
            max => 3,
        },
        {
            min => 1,
            max => 3,
        },
        {
            trimmed_min => 1,
            max => 3,
        },
        {
            trimmed_min => 1,
            min => 2,
            max => 3,
        },
    }
}
