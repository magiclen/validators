#[macro_use]
extern crate validators;
#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::collections::HashSet;

use regex::Regex;

#[test]
fn validated_customized_string() {
    validated_customized_string!(S1,
        from_string input {
            Ok(input)
        },
        from_str input {
            Ok(input.to_string())
        }
    );

    validated_customized_string!(pub S2,
        from_string input {
            Ok(input)
        },
        from_str input {
            Ok(input.to_string())
        }
    );
}

#[test]
fn validated_customized_regex_string() {
    validated_customized_regex_string!(S1, "^(Hi|Hello)$");
    validated_customized_regex_string!(pub S2, r"^[\S\s]+$");
}

#[test]
fn validated_customized_regex_string_static() {
    lazy_static! {
        static ref RE_S1: Regex = {
            Regex::new("^(Hi|Hello)$").unwrap()
        };
    }

    lazy_static! {
        static ref RE_S2: Regex = {
            Regex::new(r"^[\S\s]+$").unwrap()
        };
    }

    validated_customized_regex_string!(S1, ref RE_S1);
    validated_customized_regex_string!(pub S2, ref RE_S2);
}

#[test]
fn validated_customized_number() {
    validated_customized_number!(N1, u8,
        from_string _input {
            Ok(5)
        },
        from_str _input {
            Ok(6)
        },
        from_number input {
            Ok(input)
        }
    );

    validated_customized_number!(pub N2, u16,
        from_string _input {
            Ok(5)
        },
        from_str _input {
            Ok(6)
        },
        from_number input {
            Ok(input)
        }
    );
}

#[test]
fn validated_customized_regex_number() {
    validated_customized_regex_number!(N1, u8, r"^[1-8][0-9]$");
    validated_customized_regex_number!(pub N2, u16, r"^[0-1]?[1-8][0-9]$");
    validated_customized_regex_number!(N3, f32, r"^[0-1]?[1-8][0-9]\.[0-9]$");
}

#[test]
fn validated_customized_regex_number_static() {
    lazy_static! {
        static ref RE_N1: Regex = {
            Regex::new(r"^[1-8][0-9]$").unwrap()
        };
    }

    lazy_static! {
        static ref RE_N2: Regex = {
            Regex::new(r"^[0-1]?[1-8][0-9]$").unwrap()
        };
    }

    lazy_static! {
        static ref RE_N3: Regex = {
            Regex::new(r"^[0-1]?[1-8][0-9]\.[0-9]$").unwrap()
        };
    }

    validated_customized_regex_number!(N1, u8, ref RE_N1);
    validated_customized_regex_number!(pub N2, u16, ref RE_N2);
    validated_customized_regex_number!(N3, f32, ref RE_N3);
}

#[test]
fn validated_customized_ranged_number() {
    validated_customized_ranged_number!(N1, u8, 0, 100);
    validated_customized_ranged_number!(pub N2, u16, 3, 46);
    validated_customized_ranged_number!(N3, f32, -45.5, 80.0);
}

#[test]
fn validated_customized_primitive_number() {
    validated_customized_primitive_number!(N1, u8);
    validated_customized_primitive_number!(pub N2, u8);
    validated_customized_primitive_number!(N3, f32);
}

#[test]
fn validated_customized_vec() {
    validated_customized_vec!(V1,
        from_string _input {
            Ok(Vec::new())
        },
        from_str _input {
            Ok(Vec::new())
        },
        from_vec input {
            Ok(input)
        }
    );

    validated_customized_vec!(pub V2,
        from_string _input {
            Ok(Vec::new())
        },
        from_str _input {
            Ok(Vec::new())
        },
        from_vec input {
            Ok(input)
        }
    );
}

#[test]
fn validated_customized_ranged_length_vec() {
    validated_customized_ranged_length_vec!(V1, 0, 10);
    validated_customized_ranged_length_vec!(V2, 5);
    validated_customized_ranged_length_vec!(pub V3, 0, 10);
    validated_customized_ranged_length_vec!(pub V4, 5);
}

#[test]
fn validated_customized_hash_set() {
    validated_customized_hash_set!(S1,
        from_string _input {
            Ok(HashSet::new())
        },
        from_str _input {
            Ok(HashSet::new())
        },
        from_hash_set input {
            Ok(input)
        }
    );

    validated_customized_hash_set!(pub S2,
        from_string _input {
            Ok(HashSet::new())
        },
        from_str _input {
            Ok(HashSet::new())
        },
        from_hash_set input {
            Ok(input)
        }
    );
}

#[test]
fn validated_customized_ranged_length_hash_set() {
    validated_customized_ranged_length_hash_set!(S1, 0, 10);
    validated_customized_ranged_length_hash_set!(S2, 5);
    validated_customized_ranged_length_hash_set!(pub S3, 0, 10);
    validated_customized_ranged_length_hash_set!(pub S4, 5);
}