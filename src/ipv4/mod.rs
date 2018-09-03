extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated, ValidatedWrapper};

use std::fmt::{self, Display, Debug, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::{Utf8Error, FromStr};

fn is_local_ipv4(addr: &Ipv4Addr) -> bool {
    addr.is_private() || addr.is_loopback() || addr.is_link_local() || addr.is_broadcast() || addr.is_documentation() || addr.is_unspecified()
}

#[derive(Debug, PartialEq, Clone)]
pub enum IPv4Error {
    IncorrectFormat,
    IncorrectPort,
    PortNotAllow,
    PortNotFound,
    LocalNotAllow,
    LocalNotFound,
    IPv6NotAllow,
    IPv6NotFound,
    UTF8Error(Utf8Error),
}

pub type IPv4Result = Result<IPv4, IPv4Error>;

pub struct IPv4Validator {
    pub port: ValidatorOption,
    pub local: ValidatorOption,
    pub ipv6: ValidatorOption,
}

pub type IPv4Port = u16;

#[derive(Clone)]
pub struct IPv4 {
    ip: Ipv4Addr,
    port: u16,
    port_index: usize,
    full_ipv4: String,
    full_ipv4_len: usize,
    is_local: bool,
}

impl IPv4 {
    pub fn get_ipv4_address(&self) -> &Ipv4Addr {
        &self.ip
    }

    pub fn get_port(&self) -> Option<u16> {
        if self.port_index != self.full_ipv4_len {
            Some(self.port)
        } else {
            None
        }
    }

    pub fn get_full_ipv4(&self) -> &str {
        &self.full_ipv4
    }

    pub fn get_full_ipv4_without_port(&self) -> &str {
        if self.port_index != self.full_ipv4_len {
            &self.full_ipv4[..(self.port_index - 1)]
        } else {
            &self.full_ipv4
        }
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }
}

impl Validated for IPv4 {}

impl Debug for IPv4 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("IPv4({})", self.full_ipv4))?;
        Ok(())
    }
}

impl Display for IPv4 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_ipv4)?;
        Ok(())
    }
}

impl PartialEq for IPv4 {
    fn eq(&self, other: &Self) -> bool {
        if self.port != other.port {
            return false;
        }

        self.ip.eq(&other.ip)
    }

    fn ne(&self, other: &Self) -> bool {
        if self.port != other.port {
            return true;
        }

        self.ip.ne(&other.ip)
    }
}

impl IPv4Validator {
    pub fn is_ipv4(&self, full_ipv4: &str) -> bool {
        self.parse_inner(full_ipv4).is_ok()
    }

    pub fn parse_string(&self, full_ipv4: String) -> IPv4Result {
        let mut ipv4_inner = self.parse_inner(&full_ipv4)?;

        if ipv4_inner.full_ipv4_len != 0 {
            ipv4_inner.full_ipv4 = full_ipv4;
        } else {
            let ipv4 = ipv4_inner.ip.to_string();
            let len = ipv4.len();

            if ipv4_inner.port_index == 0 {
                ipv4_inner.full_ipv4 = ipv4;
                ipv4_inner.full_ipv4_len = len;
                ipv4_inner.port_index = len;
            } else {
                let full_ipv4 = format!("{}:{}", ipv4, ipv4_inner.port);
                let full_ipv4_len = ipv4.len();
                ipv4_inner.full_ipv4 = full_ipv4;
                ipv4_inner.full_ipv4_len = full_ipv4_len;
                ipv4_inner.port_index = len + 1;
            }
        }

        Ok(ipv4_inner)
    }

    pub fn parse_str(&self, full_ipv4: &str) -> IPv4Result {
        let mut ipv4_inner = self.parse_inner(&full_ipv4)?;

        if ipv4_inner.full_ipv4_len != 0 {
            ipv4_inner.full_ipv4 = full_ipv4.to_string();
        } else {
            let ipv4 = ipv4_inner.ip.to_string();
            let len = ipv4.len();

            if ipv4_inner.port_index == 0 {
                ipv4_inner.full_ipv4 = ipv4;
                ipv4_inner.full_ipv4_len = len;
                ipv4_inner.port_index = len;
            } else {
                let full_ipv4 = format!("{}:{}", ipv4, ipv4_inner.port);
                let full_ipv4_len = ipv4.len();
                ipv4_inner.full_ipv4 = full_ipv4;
                ipv4_inner.full_ipv4_len = full_ipv4_len;
                ipv4_inner.port_index = len + 1;
            }
        }

        Ok(ipv4_inner)
    }

    fn parse_inner(&self, ipv4: &str) -> IPv4Result {
        let re_ipv4 = Regex::new(r"^((25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9]))(:(\d{1,5}))?$").unwrap();

        let mut port = 0u16;
        let mut port_index = 0;
        let mut full_ipv4_len = 0usize;

        let ip = match re_ipv4.captures(&ipv4) {
            Some(c) => {
                if self.ipv6.must() {
                    return Err(IPv4Error::IPv6NotFound);
                }

                match c.get(11) {
                    Some(m) => {
                        if self.port.not_allow() {
                            return Err(IPv4Error::PortNotAllow);
                        }

                        port = match ipv4[m.start()..m.end()].parse::<u16>() {
                            Ok(p) => {
                                port_index = m.start();
                                p
                            }
                            Err(_) => return Err(IPv4Error::IncorrectPort)
                        };
                    }
                    None => {
                        if self.port.must() {
                            return Err(IPv4Error::PortNotFound);
                        }
                        port_index = ipv4.len();
                    }
                };

                match c.get(1) {
                    Some(m) => {
                        full_ipv4_len = 1;
                        Ipv4Addr::from_str(&ipv4[m.start()..m.end()]).map_err(|_| IPv4Error::IncorrectFormat)?
                    }
                    None => {
                        panic!("impossible");
                    }
                }
            }
            None => {
                if ipv4.starts_with("[") {
                    let re_ipv6 = Regex::new(r"^\[(([0-9a-fA-F.]{1,4})(:[0-9a-fA-F.]{1,4}){0,7})](:(\d{1,5}))?$").unwrap();

                    let c = match re_ipv6.captures(&ipv4) {
                        Some(c) => c,
                        None => {
                            return Err(IPv4Error::IncorrectFormat);
                        }
                    };

                    match c.get(5) {
                        Some(m) => {
                            if self.port.not_allow() {
                                return Err(IPv4Error::PortNotAllow);
                            }

                            port = match ipv4[m.start()..m.end()].parse::<u16>() {
                                Ok(p) => {
                                    port_index = m.start();
                                    p
                                }
                                Err(_) => return Err(IPv4Error::IncorrectPort)
                            };
                        }
                        None => {
                            if self.port.must() {
                                return Err(IPv4Error::PortNotFound);
                            }
                        }
                    };

                    match c.get(1) {
                        Some(m) => {
                            let ipv6 = Ipv6Addr::from_str(&ipv4[m.start()..m.end()]).map_err(|_| IPv4Error::IncorrectFormat)?;

                            if self.ipv6.not_allow() {
                                return Err(IPv4Error::IPv6NotAllow);
                            }

                            match ipv6.to_ipv4() {
                                Some(ip) => ip,
                                None => return Err(IPv4Error::IncorrectFormat)
                            }
                        }
                        None => {
                            panic!("impossible");
                        }
                    }
                } else {
                    let re_ipv6 = Regex::new(r"^(([0-9a-fA-F.]{1,4})(:[0-9a-fA-F.]{1,4}){0,7})$").unwrap();

                    let c = match re_ipv6.captures(&ipv4) {
                        Some(c) => c,
                        None => {
                            return Err(IPv4Error::IncorrectFormat);
                        }
                    };

                    match c.get(1) {
                        Some(m) => {
                            let ipv6 = Ipv6Addr::from_str(&ipv4[m.start()..m.end()]).map_err(|_| IPv4Error::IncorrectFormat)?;

                            if self.ipv6.not_allow() {
                                return Err(IPv4Error::IPv6NotAllow);
                            }

                            match ipv6.to_ipv4() {
                                Some(ip) => ip,
                                None => return Err(IPv4Error::IncorrectFormat)
                            }
                        }
                        None => {
                            panic!("impossible");
                        }
                    }
                }
            }
        };

        let is_local = is_local_ipv4(&ip);

        match self.local {
            ValidatorOption::Must => {
                if !is_local {
                    return Err(IPv4Error::LocalNotFound);
                }
            }
            ValidatorOption::NotAllow => {
                if is_local {
                    return Err(IPv4Error::LocalNotAllow);
                }
            }
            _ => ()
        }

        Ok(IPv4 {
            ip,
            port,
            port_index,
            full_ipv4: String::new(),
            full_ipv4_len,
            is_local,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_methods() {
        let ip = "168.17.212.1:8080".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::NotAllow,
        };

        let ipv4 = iv.parse_string(ip).unwrap();

        assert_eq!("168.17.212.1:8080", ipv4.get_full_ipv4());
        assert_eq!("168.17.212.1", ipv4.get_full_ipv4_without_port());
        assert_eq!(8080, ipv4.get_port().unwrap());
        assert_eq!(false, ipv4.is_local());
    }

    #[test]
    fn test_ipv4_lv1() {
        let ip = "168.17.212.1".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::NotAllow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv4_lv2() {
        let ip = "127.0.0.1".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::Allow,
            ipv6: ValidatorOption::NotAllow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv4_lv3() {
        let ip = "168.17.212.1:8080".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::NotAllow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv4_lv4() {
        let ip = "0000:0000:0000:0000:0000:0000:370:7348".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::Allow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv4_lv5() {
        let ip = "[0000:0000:0000:0000:0000:0000:370:7348]".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::Allow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv4_lv6() {
        let ip = "[0000:0000:0000:0000:0000:0000:370:7348]:8080".to_string();

        let iv = IPv4Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv6: ValidatorOption::Allow,
        };

        iv.parse_string(ip).unwrap();
    }
}

// TODO ----------

macro_rules! extend {
    ( $name:ident, $port:expr, $local:expr, $ipv6:expr ) => {
        #[derive(Clone)]
        pub struct $name(IPv4);

        impl From<$name> for IPv4 {
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = IPv4Error;

            fn from_string(ipv4: String) -> Result<Self, Self::Error>{
                $name::from_string(ipv4)
            }

            fn from_str(ipv4: &str) -> Result<Self, Self::Error>{
                $name::from_str(ipv4)
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                f.write_fmt(format_args!("{}({})", stringify!($name), self.0))?;
                Ok(())
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                Display::fmt(&self.0, f)
            }
        }

        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool {
                self.0.eq(&other.0)
            }

            fn ne(&self, other: &Self) -> bool {
                self.0.ne(&other.0)
            }
        }

        impl PartialEq<IPv4> for $name {
            fn eq(&self, other: &IPv4) -> bool {
                self.0.eq(&other)
            }

            fn ne(&self, other: &IPv4) -> bool {
                self.0.ne(&other)
            }
        }

        impl $name {
            pub fn from_string(ipv4: String) -> Result<$name, IPv4Error> {
                let ic = IPv4Validator {
                    port: $port,
                    local: $local,
                    ipv6: $ipv6,
                };

                Ok($name(ic.parse_string(ipv4)?))
            }

            pub fn from_str(ipv4: &str) -> Result<$name, IPv4Error> {
                let ic = IPv4Validator {
                    port: $port,
                    local: $local,
                    ipv6: $ipv6,
                };

                Ok($name(ic.parse_str(ipv4)?))
            }

            pub fn from_ipv4(ipv4: IPv4) -> Result<$name, IPv4Error> {
                 match $port {
                    ValidatorOption::Must => {
                        if ipv4.port_index == ipv4.full_ipv4_len {
                            return Err(IPv4Error::PortNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if ipv4.port_index != ipv4.full_ipv4_len {
                            return Err(IPv4Error::PortNotAllow)
                        }
                    }
                    _=>()
                }
                match $local {
                    ValidatorOption::Must => {
                        if !ipv4.is_local {
                            return Err(IPv4Error::LocalNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if ipv4.is_local {
                            return Err(IPv4Error::LocalNotAllow)
                        }
                    }
                    _=>()
                }

                Ok($name(ipv4))
            }

            pub fn into_ipv4(self) -> IPv4 {
                self.0
            }

            pub fn as_ipv4(&self) -> &IPv4 {
                &self.0
            }
        }

        impl $name {
            pub fn get_ipv4_address(&self) -> &Ipv4Addr {
                &self.0.ip
            }

            pub fn get_full_ipv4(&self) -> &str {
                &self.0.full_ipv4
            }

            pub fn get_full_ipv4_without_port(&self) -> &str {
                if self.0.port_index != self.0.full_ipv4_len {
                    &self.0.full_ipv4[..(self.0.port_index - 1)]
                } else {
                    &self.0.full_ipv4
                }
            }
        }

         #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = IPv4Error;

            fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| IPv4Error::UTF8Error(err))?)
            }
        }
    };
}

extend!(IPv4LocalableWithPort, ValidatorOption::Must, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv4LocalableWithPort {
    pub fn get_port(&self) -> u16 {
        self.0.port
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv4LocalableAllowPort, ValidatorOption::Allow, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv4LocalableAllowPort {
    pub fn get_port(&self) -> Option<u16> {
        if self.0.port_index != self.0.full_ipv4_len {
            Some(self.0.port)
        } else {
            None
        }
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv4LocalableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv4LocalableWithoutPort {
    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv4UnlocalableWithPort, ValidatorOption::Must, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv4UnlocalableWithPort {
    pub fn get_port(&self) -> u16 {
        self.0.port
    }
}

extend!(IPv4UnlocalableAllowPort, ValidatorOption::Allow, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv4UnlocalableAllowPort {
    pub fn get_port(&self) -> Option<u16> {
        if self.0.port_index != self.0.full_ipv4_len {
            Some(self.0.port)
        } else {
            None
        }
    }
}

extend!(IPv4UnlocalableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv4UnlocalableWithoutPort {}