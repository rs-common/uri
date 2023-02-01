use std::io::SeekFrom;
use std::net::{Ipv4Addr, Ipv6Addr};
use std::str::FromStr;
use regex::Regex;
use super::encoder::{Decoder, Encoder};
use super::Parser;
use super::error::{Error, Result};

#[derive(Debug, Clone)]
pub struct Authority {
    user_info: Option<UserInfo>,
    host: Host,
    port: Option<Port>,
}

impl Authority {
    pub fn new(host: Host) -> Self {
        Authority {
            user_info: None,
            host,
            port: None,
        }
    }

    pub fn set_userinfo(&mut self, info: UserInfo) -> &mut Self {
        self.user_info = Some(info);
        self
    }

    pub fn set_port(&mut self, port: Port) -> &mut Self {
        self.port = Some(port);
        self
    }
}

impl Parser for Authority {
    fn decode(s: &str) -> Result<Self> {
        let mut aut = Authority {
            user_info: None,
            host: Host::RegName(String::new()),
            port: None,
        };
        let mut rest = s.to_owned();
        if let Some(uinfo_index) = rest.find('@') {
            let right = rest.split_off(uinfo_index);
            let u = UserInfo::decode(&rest)?;
            aut.user_info = Some(u.clone());
            rest = right;
        }
        if let Some(port_index) = rest.rfind(':') {
            let right = rest.split_off(port_index);
            let right = right.trim_start_matches(":");
            let u = Port::decode(&right)?;
            aut.port = Some(u.clone());
        }
        let rest = rest.trim_start_matches("@");
        let u = Host::decode(rest)?;
        aut.host = u.clone();
        Ok(aut)
    }

    fn encode(&self) -> Result<String> {
        let mut buf = String::new();
        let mut uinfo = String::new();
        let mut host = String::new();
        let mut port = String::new();
        if let Some(c) = &self.user_info {
            uinfo = c.encode()?;
        }
        host = self.host.encode()?;
        if let Some(c) = &self.port {
            port = c.encode()?;
        }
        if host.len() > 0 {
            if uinfo.len() > 0 {
                buf.push_str(format!("{}@", uinfo).as_str())
            }
            buf.push_str(host.as_str());
            if port.len() > 0 {
                buf.push_str(format!(":{}", port).as_str())
            }
        }
        Ok(buf)
    }
}

#[derive(Debug, Clone)]
pub struct UserInfo(String);

impl UserInfo {
    pub fn new(s: &str) -> Self {
        UserInfo(s.to_string())
    }
}

impl Parser for UserInfo {
    fn decode(s: &str) -> Result<Self> {
        let mut dec = Decoder::new(s);
        dec.set_decode_pct();
        dec.allowed().set_unreserved().set_subdelims().set(vec![b':']);
        let s = dec.decode()?;
        Ok(UserInfo(s))
    }

    fn encode(&self) -> Result<String> {
        let mut enc = Encoder::new(self.0.as_str());
        enc.set_encode_pct();
        enc.allowed().set_unreserved().set_subdelims().set(vec![b':']);
        let s = enc.encode()?;
        Ok(s)
    }
}

#[derive(Debug, Clone)]
pub enum Host {
    IPLiteral(String),
    IPv4Addr(Ipv4Addr),
    RegName(String),
}

impl Host {
    fn decode_ipliteral(s: &str) -> Result<Host> {
        let s = &s[1..s.len() - 1];
        if s.len() == 0 {
            return Err(Error::Encode("invalid host string, is cannot be empty".to_string()));
        }
        // ipv6地址
        if let Ok(v6addr) = Ipv6Addr::from_str(s) {
            return Ok(Host::IPLiteral(v6addr.to_string()));
        }
        // ipvfuture
        let ipv_future: Regex =
            Regex::new(r#"v[[:xdigit:]]+\.([[:alnum:]]|[!$&'\(\)*+,;=:])+"#)?;
        if ipv_future.is_match(s) {
            return Ok(Host::IPLiteral(s.to_string()));
        }
        Err(Error::Encode("not a valid ip-literal string".to_string()))
    }

    fn encode_ipliteral(s: &str) -> Result<String> {
        if let Ok(v6addr) = Ipv6Addr::from_str(s) {
            return Ok(v6addr.to_string());
        }
        let ipv_future: Regex =
            Regex::new(r#"v[[:xdigit:]]+\.([[:alnum:]]|[!$&'\(\)*+,;=:])+"#)?;
        if ipv_future.is_match(s) {
            return Ok(s.to_string());
        }
        Err(Error::Encode(format!("invalid ip literal string,\'{}\'", s)))
    }

    fn decode_reg_name(s: &str) -> Result<Host> {
        let mut dec = Decoder::new(s);
        dec.allow_empty();
        dec.set_decode_pct();
        dec.allowed().set_unreserved().set_subdelims();
        match dec.decode() {
            Ok(s) => Ok(Host::RegName(s)),
            Err(e) => Err(Error::Encode(format!("not a valid reg-name host, err={}", e))),
        }
    }

    fn encode_reg_name(s: &str) -> Result<String> {
        let mut enc = Encoder::new(s);
        enc.allow_empty();
        enc.allowed().set_unreserved().set_subdelims();
        enc.set_encode_pct();
        enc.encode()
    }
}

impl Parser for Host {
    fn decode(s: &str) -> Result<Host> {
        let sbytes = s.as_bytes();
        // IPLiteral
        if sbytes.len() > 2 && sbytes[0] == b'[' && sbytes[sbytes.len() - 1] == b']' {
            return Host::decode_ipliteral(s);
        }
        // IPv4Addr
        if let Ok(v4addr) = Ipv4Addr::from_str(s) {
            return Ok(Host::IPv4Addr(v4addr));
        }
        // RegName
        if let Ok(rname) = Host::decode_reg_name(s) {
            return Ok(rname);
        }
        Err(Error::Encode(format!("not a valid host string \'{}\'", s)))
    }

    fn encode(&self) -> Result<String> {
        match self {
            Host::IPLiteral(lit) => Host::encode_ipliteral(lit.as_str()),
            Host::IPv4Addr(v4addr) => Ok(v4addr.to_string()),
            Host::RegName(regname) => Host::encode_reg_name(regname),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Port(u16);

impl Port {
    pub fn new(p: u16) -> Self {
        Port(p)
    }
}

impl Parser for Port {
    fn decode(s: &str) -> Result<Self> {
        let port = u16::from_str(s)?;
        Ok(Port(port))
    }

    fn encode(&self) -> Result<String> {
        Ok(self.0.to_string())
    }
}