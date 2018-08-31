extern crate regex;

use self::regex::Regex;
use super::{ValidatorOption, Validated};

use std::fmt::{self, Display, Formatter};
use std::net::{Ipv4Addr, Ipv6Addr};
#[cfg(feature = "nightly")]
use std::net::Ipv6MulticastScope;
use std::str::FromStr;

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

#[derive(Debug)]
pub enum IPv6Error {
    IncorrectFormat,
    IncorrectPort,
    PortNotAllow,
    PortNotFound,
    LocalNotAllow,
    LocalNotFound,
    IPv4NotAllow,
    IPv4NotFound,
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
    port: Option<u16>,
    is_local: bool,
}

impl IPv6 {
    pub fn get_ipv6_address(&self) -> &Ipv6Addr {
        &self.ip
    }

    pub fn get_port(&self) -> Option<u16> {
        self.port
    }

    pub fn get_full_address(&self) -> String {
        match self.port {
            Some(p) => {
                let mut s = self.ip.to_string();
                s.push_str(":");
                s.push_str(&p.to_string());
                s
            }
            None => self.ip.to_string()
        }
    }

    pub fn is_local(&self) -> bool {
        self.is_local
    }
}

impl Validated for IPv6 {}

impl Display for IPv6 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.port {
            Some(p) => {
                f.write_str(&self.ip.to_string())?;
                f.write_str(":")?;
                f.write_str(&p.to_string())?;
            }
            None => f.write_str(&self.ip.to_string())?
        }
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
    pub fn is_ipv6(&self, ipv6: &str) -> bool {
        self.parse_inner(ipv6).is_ok()
    }

    pub fn parse_string(&self, ipv6: String) -> IPv6Result {
        self.parse_inner(&ipv6)
    }

    pub fn parse_str(&self, ipv6: &str) -> IPv6Result {
        self.parse_inner(ipv6)
    }

    fn parse_inner(&self, ipv6: &str) -> IPv6Result {
        let mut port = None;

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
                        Ok(p) => Some(p),
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
                                        Ok(p) => Some(p),
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
            is_local,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv6_lv1() {
        let domain = "168.17.212.1".to_string();

        let dv = IPv6Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::Allow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_ipv6_lv2() {
        let domain = "127.0.0.1".to_string();

        let dv = IPv6Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::Allow,
            ipv4: ValidatorOption::Allow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_ipv6_lv3() {
        let domain = "168.17.212.1:8080".to_string();

        let dv = IPv6Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::Allow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_ipv6_lv4() {
        let domain = "0000:0000:0000:0000:0000:0000:370:7348".to_string();

        let dv = IPv6Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_ipv6_lv5() {
        let domain = "[0000:0000:0000:0000:0000:0000:370:7348]".to_string();

        let dv = IPv6Validator {
            port: ValidatorOption::NotAllow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_ipv6_lv6() {
        let domain = "[0000:0000:0000:0000:0000:0000:370:7348]:8080".to_string();

        let dv = IPv6Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        dv.parse_string(domain).unwrap();
    }

    #[test]
    fn test_ipv6_lv7() {
        let domain = "[FF:8888:1234:0000:0000:0000:370:7348]:8080".to_string();

        let dv = IPv6Validator {
            port: ValidatorOption::Allow,
            local: ValidatorOption::NotAllow,
            ipv4: ValidatorOption::NotAllow,
        };

        dv.parse_string(domain).unwrap();
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

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> fmt::Result {
                self.0.fmt(f)
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
                        if let None = ipv6.port {
                            return Err(IPv6Error::PortNotFound)
                        }
                    },
                    ValidatorOption::NotAllow => {
                        if let Some(_) = ipv6.port {
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

            pub fn get_ipv6(&self) -> &IPv6 {
                &self.0
            }
        }
    };
}

extend!(IPv6LocalableWithPort, ValidatorOption::Must, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv6LocalableWithPort {
    pub fn get_ipv6_address(&self) -> &Ipv6Addr {
        &self.0.ip
    }

    pub fn get_port(&self) -> u16 {
        self.0.port.unwrap()
    }

    pub fn get_full_address(&self) -> String {
        match self.0.port {
            Some(p) => {
                let mut s = self.0.ip.to_string();
                s.push_str(":");
                s.push_str(&p.to_string());
                s
            }
            None => {
                panic!("impossible")
            }
        }
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv6LocalableAllowPort, ValidatorOption::Allow, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv6LocalableAllowPort {
    pub fn get_ipv4_address(&self) -> &Ipv6Addr {
        &self.0.ip
    }

    pub fn get_port(&self) -> Option<u16> {
        self.0.port
    }

    pub fn get_full_address(&self) -> String {
        match self.0.port {
            Some(p) => {
                let mut s = self.0.ip.to_string();
                s.push_str(":");
                s.push_str(&p.to_string());
                s
            }
            None => self.0.ip.to_string()
        }
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv6LocalableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::Allow, ValidatorOption::Allow);

impl IPv6LocalableWithoutPort {
    pub fn get_ipv4_address(&self) -> &Ipv6Addr {
        &self.0.ip
    }

    pub fn get_full_address(&self) -> String {
        match self.0.port {
            Some(_) => {
                panic!("impossible")
            }
            None => self.0.ip.to_string()
        }
    }

    pub fn is_local(&self) -> bool {
        self.0.is_local
    }
}

extend!(IPv6UnlocalableWithPort, ValidatorOption::Must, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv6UnlocalableWithPort {
    pub fn get_ipv6_address(&self) -> &Ipv6Addr {
        &self.0.ip
    }

    pub fn get_port(&self) -> u16 {
        self.0.port.unwrap()
    }

    pub fn get_full_address(&self) -> String {
        match self.0.port {
            Some(p) => {
                let mut s = self.0.ip.to_string();
                s.push_str(":");
                s.push_str(&p.to_string());
                s
            }
            None => {
                panic!("impossible")
            }
        }
    }
}

extend!(IPv6UnlocalableAllowPort, ValidatorOption::Allow, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv6UnlocalableAllowPort {
    pub fn get_ipv6_address(&self) -> &Ipv6Addr {
        &self.0.ip
    }

    pub fn get_port(&self) -> Option<u16> {
        self.0.port
    }

    pub fn get_full_address(&self) -> String {
        match self.0.port {
            Some(p) => {
                let mut s = self.0.ip.to_string();
                s.push_str(":");
                s.push_str(&p.to_string());
                s
            }
            None => self.0.ip.to_string()
        }
    }
}

extend!(IPv6UnlocalableWithoutPort, ValidatorOption::NotAllow, ValidatorOption::NotAllow, ValidatorOption::Allow);

impl IPv6UnlocalableWithoutPort {
    pub fn get_ipv6_address(&self) -> &Ipv6Addr {
        &self.0.ip
    }

    pub fn get_full_address(&self) -> String {
        match self.0.port {
            Some(_) => {
                panic!("impossible")
            }
            None => self.0.ip.to_string()
        }
    }
}