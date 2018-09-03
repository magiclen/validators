extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated, ValidatedWrapper};

use std::fmt::{self, Display, Debug, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
#[cfg(feature = "nightly")]
use std::net::Ipv6MulticastScope;
use std::str::{Utf8Error, FromStr};

#[cfg(not(feature = "nightly"))]
fn is_local_ipv6(addr: &Ipv6Addr) -> bool {
    addr.is_multicast() || addr.is_loopback() || addr.is_unspecified()
}

#[cfg(feature = "nightly")]
fn is_local_ipv6(addr: &Ipv6Addr) -> bool {
    match addr.multicast_scope() {
        Some(Ipv6MulticastScope::Global) => false,
        None => addr.is_multicast() || addr.is_loopback() || addr.is_unicast_link_local() || addr.is_unicast_site_local() || addr.is_unique_local() || addr.is_unspecified() || addr.is_documentation(),
        _ => true
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum IPv6Error {
    IncorrectFormat,
    IncorrectPort,
    PortNotAllow,
    PortNotFound,
    LocalNotAllow,
    LocalNotFound,
    IPv4NotAllow,
    IPv4NotFound,
    UTF8Error(Utf8Error),
}

pub type IPv6Result = Result<IPv6, IPv6Error>;

pub struct IPv6Validator {
    pub port: ValidatorOption,
    pub local: ValidatorOption,
    pub ipv4: ValidatorOption,
}

pub type IPv6Port = u16;

#[derive(Clone)]
pub struct IPv6 {
    ip: Ipv6Addr,
    port: u16,
    port_index: usize,
    full_ipv6: String,
    full_ipv6_len: usize,
    is_local: bool,
}

impl IPv6 {
    pub fn get_ipv6_address(&self) -> &Ipv6Addr {
        &self.ip
    }

    pub fn get_port(&self) -> Option<u16> {
        if self.port_index != self.full_ipv6_len {
            Some(self.port)
        } else {
            None
        }
    }

    pub fn get_full_ipv6(&self) -> &str {
        &self.full_ipv6
    }

    pub fn get_full_ipv6_without_port(&self) -> &str {
        if self.port_index != self.full_ipv6_len {
            &self.full_ipv6[1..(self.port_index - 2)]
        } else {
            &self.full_ipv6
        }
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }
}

impl Validated for IPv6 {}

impl Debug for IPv6 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("IPv6({})", self.full_ipv6))?;
        Ok(())
    }
}

impl Display for IPv6 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_str(&self.full_ipv6)?;
        Ok(())
    }
}

impl PartialEq for IPv6 {
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

impl IPv6Validator {
    pub fn is_ipv6(&self, full_ipv6: &str) -> bool {
        self.parse_inner(full_ipv6).is_ok()
    }

    pub fn parse_string(&self, full_ipv6: String) -> IPv6Result {
        let mut ipv6_inner = self.parse_inner(&full_ipv6)?;

        if ipv6_inner.full_ipv6_len != 0 {
            ipv6_inner.full_ipv6 = full_ipv6;
        } else {
            let ipv6 = ipv6_inner.ip.to_string();
            let len = ipv6.len();

            if ipv6_inner.port_index == 0 {
                ipv6_inner.full_ipv6 = ipv6;
                ipv6_inner.full_ipv6_len = len;
                ipv6_inner.port_index = len;
            } else {
                let full_ipv6 = format!("[{}]:{}", ipv6, ipv6_inner.port);
                let full_ipv6_len = ipv6.len();
                ipv6_inner.full_ipv6 = full_ipv6;
                ipv6_inner.full_ipv6_len = full_ipv6_len;
                ipv6_inner.port_index = len + 2;
            }
        }

        Ok(ipv6_inner)
    }

    pub fn parse_str(&self, full_ipv6: &str) -> IPv6Result {
        let mut ipv6_inner = self.parse_inner(&full_ipv6)?;

        if ipv6_inner.full_ipv6_len != 0 {
            ipv6_inner.full_ipv6 = full_ipv6.to_string();
        } else {
            let ipv6 = ipv6_inner.ip.to_string();
            let len = ipv6.len();

            if ipv6_inner.port_index == 0 {
                ipv6_inner.full_ipv6 = ipv6;
                ipv6_inner.full_ipv6_len = len;
                ipv6_inner.port_index = len;
            } else {
                let full_ipv6 = format!("[{}]:{}", ipv6, ipv6_inner.port);
                let full_ipv6_len = ipv6.len();
                ipv6_inner.full_ipv6 = full_ipv6;
                ipv6_inner.full_ipv6_len = full_ipv6_len;
                ipv6_inner.port_index = len + 2;
            }
        }

        Ok(ipv6_inner)
    }

    fn parse_inner(&self, ipv6: &str) -> IPv6Result {
        let mut port = 0u16;
        let mut port_index = 0;
        let mut full_ipv6_len = 0usize;

        let ip = if ipv6.starts_with("[") {
            let re_ipv6 = Regex::new(r"^\[(([0-9a-fA-F.]{1,4})(:[0-9a-fA-F.]{1,4}){0,7})](:(\d{1,5}))?$").unwrap();

            let c = match re_ipv6.captures(&ipv6) {
                Some(c) => c,
                None => {
                    return Err(IPv6Error::IncorrectFormat);
                }
            };

            match c.get(5) {
                Some(m) => {
                    if self.port.not_allow() {
                        return Err(IPv6Error::PortNotAllow);
                    }

                    port = match ipv6[m.start()..m.end()].parse::<u16>() {
                        Ok(p) => {
                            port_index = m.start();
                            p
                        },
                        Err(_) => return Err(IPv6Error::IncorrectPort)
                    };

                    full_ipv6_len = 1;
                }
                None => {
                    if self.port.must() {
                        return Err(IPv6Error::PortNotFound);
                    }
                }
            };

            match c.get(1) {
                Some(m) => {
                    let ipv6 = Ipv6Addr::from_str(&ipv6[m.start()..m.end()]).map_err(|_| IPv6Error::IncorrectFormat)?;

                    if self.ipv4.must() {
                        return Err(IPv6Error::IPv4NotFound);
                    }

                    ipv6
                }
                None => {
                    panic!("impossible");
                }
            }
        } else {
            let re_ipv6 = Regex::new(r"^(([0-9a-fA-F.]{1,4})(:[0-9a-fA-F.]{1,4}){0,7})$").unwrap();

            match re_ipv6.captures(&ipv6) {
                Some(c) => {
                    match c.get(1) {
                        Some(m) => {
                            let ipv6 = Ipv6Addr::from_str(&ipv6[m.start()..m.end()]).map_err(|_| IPv6Error::IncorrectFormat)?;

                            if self.ipv4.must() {
                                return Err(IPv6Error::IPv4NotFound);
                            }

                            full_ipv6_len = 1;

                            ipv6
                        }
                        None => {
                            panic!("impossible");
                        }
                    }
                }
                None => {
                    let re_ipv4 = Regex::new(r"^((25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9])\.(25[0-5]|2[0-4][0-9]|1([0-9]){1,2}|[1-9]?[0-9]))(:(\d{1,5}))?$").unwrap();

                    match re_ipv4.captures(&ipv6) {
                        Some(c) => {
                            if self.ipv4.not_allow() {
                                return Err(IPv6Error::IPv4NotAllow);
                            }

                            match c.get(11) {
                                Some(m) => {
                                    if self.port.not_allow() {
                                        return Err(IPv6Error::PortNotAllow);
                                    }

                                    port = match ipv6[m.start()..m.end()].parse::<u16>() {
                                        Ok(p) => {
                                            port_index = m.start();
                                            p
                                        },
                                        Err(_) => return Err(IPv6Error::IncorrectPort)
                                    };
                                }
                                None => {
                                    if self.port.must() {
                                        return Err(IPv6Error::PortNotFound);
                                    }
                                }
                            };

                            match c.get(1) {
                                Some(m) => {
                                    let ipv4 = Ipv4Addr::from_str(&ipv6[m.start()..m.end()]).map_err(|_| IPv6Error::IncorrectFormat)?;

                                    ipv4.to_ipv6_mapped()
                                }
                                None => {
                                    panic!("impossible");
                                }
                            }
                        }
                        None => {
                            return Err(IPv6Error::IncorrectFormat);
                        }
                    }
                }
            }
        };

        let is_local = is_local_ipv6(&ip);

        match self.local {
            ValidatorOption::Must => {
                if !is_local {
                    return Err(IPv6Error::LocalNotFound);
                }
            }
            ValidatorOption::NotAllow => {
                if is_local {
                    return Err(IPv6Error::LocalNotAllow);
                }
            }
            _ => ()
        }

        Ok(IPv6 {
            ip,
            port,
            port_index,
            full_ipv6: String::new(),
            full_ipv6_len,
            is_local,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_methods_1() {
        let ip = "FF:8888:1234:0000:0000:0000:370:7348".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        let ipv6 = iv.parse_string(ip).unwrap();

        assert_eq!("FF:8888:1234:0000:0000:0000:370:7348", ipv6.get_full_ipv6());
    }

    #[test]
    fn test_ipv6_methods_2() {
        let ip = "[FF:8888:1234:0000:0000:0000:370:7348]:8080".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        let ipv6 = iv.parse_string(ip).unwrap();

        assert_eq!("[FF:8888:1234:0000:0000:0000:370:7348]:8080", ipv6.get_full_ipv6());
        assert_eq!("FF:8888:1234:0000:0000:0000:370:7348", ipv6.get_full_ipv6_without_port());
        assert_eq!(8080, ipv6.get_port().unwrap());
        assert_eq!(false, ipv6.is_local());
    }

    #[test]
    fn test_ipv6_lv1() {
        let ip = "168.17.212.1".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::Allow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv6_lv2() {
        let ip = "127.0.0.1".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::Allow,
            ipv4: ValidatorOption::Allow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv6_lv3() {
        let ip = "168.17.212.1:8080".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::Allow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv6_lv4() {
        let ip = "0000:0000:0000:0000:0000:0000:370:7348".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv6_lv5() {
        let ip = "[0000:0000:0000:0000:0000:0000:370:7348]".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv6_lv6() {
        let ip = "[0000:0000:0000:0000:0000:0000:370:7348]:8080".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        iv.parse_string(ip).unwrap();
    }

    #[test]
    fn test_ipv6_lv7() {
        let ip = "[FF:8888:1234:0000:0000:0000:370:7348]:8080".to_string();

        let iv = IPv6Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        iv.parse_string(ip).unwrap();
    }
}

// TODO ----------

macro_rules! extend {
    ( $name:ident, $port:expr, $local:expr, $ipv4:expr ) => {
        #[derive(Clone)]
        pub struct $name(IPv6);

        impl From<$name> for IPv6 {
            fn from(d: $name) -> Self {
                d.0
            }
        }

        impl Validated for $name {}

        impl ValidatedWrapper for $name {
            type Error = IPv6Error;

            fn from_string(ipv6: String) -> Result<Self, Self::Error>{
                $name::from_string(ipv6)
            }

            fn from_str(ipv6: &str) -> Result<Self, Self::Error>{
                $name::from_str(ipv6)
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

        impl PartialEq<IPv6> for $name {
            fn eq(&self, other: &IPv6) -> bool {
                self.0.eq(&other)
            }

            fn ne(&self, other: &IPv6) -> bool {
                self.0.ne(&other)
            }
        }

        impl $name {
            pub fn from_string(ipv6: String) -> Result<$name, IPv6Error> {
                let ic = IPv6Validator {
                    port: $port,
                    local: $local,
                    ipv4: $ipv4,
                };

                Ok($name(ic.parse_string(ipv6)?))
            }

            pub fn from_str(ipv6: &str) -> Result<$name, IPv6Error> {
                let ic = IPv6Validator {
                    port: $port,
                    local: $local,
                    ipv4: $ipv4,
                };

                Ok($name(ic.parse_str(ipv6)?))
            }

            pub fn from_ipv6(ipv6: IPv6) -> Result<$name, IPv6Error> {
                 match $port {
                    ValidatorOption::Must => {
                        if ipv6.port_index == ipv6.full_ipv6_len {
                            return Err(IPv6Error::PortNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if ipv6.port_index != ipv6.full_ipv6_len {
                            return Err(IPv6Error::PortNotAllow)
                        }
                    }
                    _=>()
                }
                match $local {
                    ValidatorOption::Must => {
                        if !ipv6.is_local {
                            return Err(IPv6Error::LocalNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if ipv6.is_local {
                            return Err(IPv6Error::LocalNotAllow)
                        }
                    }
                    _=>()
                }

                Ok($name(ipv6))
            }

            pub fn into_ipv6(self) -> IPv6 {
                self.0
            }

            pub fn as_ipv6(&self) -> &IPv6 {
                &self.0
            }
        }

        impl $name {
            pub fn get_ipv6_address(&self) -> &Ipv6Addr {
                &self.0.ip
            }

            pub fn get_full_ipv6(&self) -> &str {
                &self.0.full_ipv6
            }

            pub fn get_full_ipv6_without_port(&self) -> &str {
                if self.0.port_index != self.0.full_ipv6_len {
                    &self.0.full_ipv6[1..(self.0.port_index - 2)]
                } else {
                    &self.0.full_ipv6
                }
            }
        }

         #[cfg(feature = "rocketly")]
        impl<'a> ::rocket::request::FromFormValue<'a> for $name {
            type Error = IPv6Error;

            fn from_form_value(form_value: &'a ::rocket::http::RawStr) -> Result<Self, Self::Error>{
                $name::from_string(form_value.url_decode().map_err(|err| IPv6Error::UTF8Error(err))?)
            }
        }
    };
}

extend!(IPv6LocalableWithPort, ValidatorOption::Must, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv6LocalableWithPort {
    pub fn get_port(&self) -> u16 {
        self.0.port
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv6LocalableAllowPort, ValidatorOption::Allow, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv6LocalableAllowPort {
    pub fn get_port(&self) -> Option<u16> {
        if self.0.port_index != self.0.full_ipv6_len {
            Some(self.0.port)
        } else {
            None
        }
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv6LocalableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv6LocalableWithoutPort {
    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv6UnlocalableWithPort, ValidatorOption::Must, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv6UnlocalableWithPort {
    pub fn get_port(&self) -> u16 {
        self.0.port
    }
}

extend!(IPv6UnlocalableAllowPort, ValidatorOption::Allow, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv6UnlocalableAllowPort {
    pub fn get_port(&self) -> Option<u16> {
        if self.0.port_index != self.0.full_ipv6_len {
            Some(self.0.port)
        } else {
            None
        }
    }
}

extend!(IPv6UnlocalableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv6UnlocalableWithoutPort {}