#![cfg(feature = "rocketly-test")]
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate validators;

#[macro_use]
extern crate rocket;

use validators::domain::DomainLocalhostableWithPort;
use validators::email::Email;

#[test]
fn from_form_value() {
    validated_customized_regex_string!(Lang, r"^(us|cn|tw)$");
    validated_customized_ranged_number!(PersonAge, u8, 0, 130);

    #[derive(FromForm)]
    struct Model1 {
        _a: DomainLocalhostableWithPort,
        _b: Email,
    }

    let _m1 = Model1 {
        _a: DomainLocalhostableWithPort::from_str("localhost:8080").unwrap(),
        _b: Email::from_str("len@magiclen.org").unwrap(),
    };

    #[derive(FromForm)]
    struct Model2 {
        _a: Lang,
    }

    let _m2 = Model2 {
        _a: Lang::from_str("tw").unwrap(),
    };

    #[derive(FromForm)]
    struct Model3 {
        _a: PersonAge,
    }

    let _m3 = Model3 {
        _a: PersonAge::from_str("18").unwrap(),
    };
}

#[test]
#[allow(dead_code)]
fn from_param() {
    validated_customized_ranged_number!(PersonID, u8, 0, 130);

    #[get("/person/<_id>")]
    fn get(_id: PersonID) -> &'static str {
        "test"
    }
}
