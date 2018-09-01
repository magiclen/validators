#![cfg(feature = "rocketly")]
#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate validators;

extern crate rocket;

use validators::domain::DomainLocalhostableWithPort;
use validators::email::Email;

validated_customized_regex_string!(Lang, r"^(us|cn|tw)$");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_form_value() {
        #[derive(FromForm)]
        struct Model1{
            a: DomainLocalhostableWithPort,
            b: Email
        }

        let m1 = Model1{
            a: DomainLocalhostableWithPort::from_str("localhost:8080").unwrap(),
            b: Email::from_str("len@magiclen.org").unwrap(),
        };

        #[derive(FromForm)]
        struct Model2{
            a: Lang,
        }

        let m2 = Model2{
            a: Lang::from_str("tw").unwrap(),
        };
    }
}