#![allow(unused)]

use core::str::utf8_char_width;
use super::error::{Error, Result};
use std::{collections::HashSet, str::Chars};
use std::fmt::format;
use std::slice::Iter;

pub struct Allowed(HashSet<u8>);

impl Allowed {
    pub fn new() -> Self {
        Allowed(HashSet::new())
    }

    pub fn set_unreserved(&mut self) -> &mut Self {
        self.set_alphanum();
        self.0.extend([b'-', b'.', b'_', b'~']);
        self
    }

    pub fn set_alphanum(&mut self) -> &mut Self {
        self.0.extend(b'a'..=b'z');
        self.0.extend(b'A'..=b'Z');
        self.0.extend(b'0'..=b'9');
        self
    }

    pub fn set_subdelims(&mut self) -> &mut Self {
        self.0.extend([b'!', b'$', b'&', b'\'', b'(', b')', b'*', b'+', b',', b';', b'=']);
        self
    }

    pub fn set_gendelims(&mut self) -> &mut Self {
        self.0.extend([b':', b'/', b'?', b'#', b'[', b']', b'@']);
        self
    }

    pub fn set(&mut self, chs: Vec<u8>) -> &mut Self {
        self.0.extend(chs);
        self
    }
}

pub struct Decoder {
    data: String,
    allowed: Allowed,
    allow_empty: bool,
    decode_pct: bool,
}

impl Decoder {
    pub fn new(s: &str) -> Self {
        Decoder {
            data: s.to_string(),
            allowed: Allowed::new(),
            allow_empty: false,
            decode_pct: false,
        }
    }

    pub fn decode(&self) -> Result<String> {
        let mut buf = Vec::<u8>::new();
        // 空判断
        if self.data.len() == 0 {
            if !self.allow_empty {
                return Err(Error::Encode("empty is not allowed".to_string()));
            }
            return Ok(String::new());
        }
        let mut iter = self.data.as_bytes().iter();
        while let Some(item) = iter.next() {
            // 是否允许的字符
            if self.allowed.0.contains(&item) {
                buf.push(*item);
                continue;
            }
            if *item == b'%' && self.decode_pct {
                let mut pct_item = pct_encode_check(&mut iter)?;
                let s = String::from_utf8(pct_item.to_vec())?;
                let u = u8::from_str_radix(s.as_str(), 16)?;
                // let width = utf8_char_width(u);
                // let decoded = pct_decode(&mut iter, width)?;
                buf.push(u);
                continue;
            }
            return Err(Error::Decode(format!("invalid decode string \'{}\'", self.data)));
        }
        let r = String::from_utf8(buf)?;
        Ok(r)
    }

    pub fn allowed(&mut self) -> &mut Allowed {
        &mut self.allowed
    }

    pub fn allow_empty(&mut self) -> &mut Self {
        self.allow_empty = true;
        self
    }

    pub fn set_decode_pct(&mut self) -> &mut Self {
        self.decode_pct = true;
        self
    }
}

pub struct Encoder {
    data: String,
    allowed: Allowed,
    allow_empty: bool,
    encode_pct: bool,
}

impl Encoder {
    pub fn new(s: &str) -> Self {
        Encoder {
            data: s.to_string(),
            allowed: Allowed::new(),
            allow_empty: false,
            encode_pct: false,
        }
    }

    pub fn encode(&self) -> Result<String> {
        let mut buf = Vec::<u8>::new();
        // 空判断
        if self.data.len() == 0 {
            if !self.allow_empty {
                return Err(Error::Encode("empty is not allowed".to_string()));
            }
            return Ok(String::new());
        }
        let mut iter = self.data.as_bytes().iter();
        while let Some(item) = iter.next() {
            // 是否允许的字符
            if self.allowed.0.contains(&item) {
                buf.push(*item);
                continue;
            }
            // 是否为百分比编码
            if *item == b'%' {
                let mut pct_item = pct_encode_check(&mut iter)?;
                buf.push(b'%');
                buf.extend(pct_item);
                continue;
            }
            // 其它字符的百分比编码
            if self.encode_pct {
                let s = format!("%{:X}", item);
                buf.extend(s.as_bytes());
                continue;
            }
            // 如果所有的都未匹配则报错
            return Err(Error::Encode(format!("invalid encode string \'{}\'", self.data)));
        }
        let r = String::from_utf8(buf)?;
        Ok(r)
    }

    pub fn is_start_with(&self, p: u8) -> bool {
        let bs = self.data.as_bytes();
        bs[0] == p
    }

    pub fn is_start_with_alpha(&self) -> bool {
        self.data.as_bytes()[0].is_ascii_alphabetic()
    }

    pub fn allowed(&mut self) -> &mut Allowed {
        &mut self.allowed
    }

    pub fn allow_empty(&mut self) -> &mut Self {
        self.allow_empty = true;
        self
    }

    pub fn set_encode_pct(&mut self) -> &mut Self {
        self.encode_pct = true;
        self
    }
}

// 检查百分比编码
fn pct_encode_check(iter: &mut Iter<u8>) -> Result<[u8; 2]> {
    let mut buf: [u8; 2] = [0; 2];
    for i in 0..=1 {
        let sub = match iter.next() {
            Some(sub) => sub,
            None => {
                return Err(Error::Encode(
                    "not a valid percent encoding char".to_string(),
                ));
            }
        };
        if !(matches!(sub,b'0'..=b'9'|b'A'..=b'F')) {
            return Err(Error::Encode(
                "percent encoding char should be a hex digit".to_string(),
            ));
        }
        buf[i] = *sub;
    }
    Ok(buf)
}

// 百分比解码
fn pct_decode(iter: &mut Iter<u8>, width: usize) -> Result<Vec<u8>> {
    let mut ret = Vec::<u8>::new();
    for i in 0..width {
        let mut buf = [0u8; 2];
        let mut j = 0;
        while j < 2 {
            if let Some(item) = iter.next() {
                if *item == b'%' {
                    continue;
                }
                buf[j] = *item;
                j += 1;
            } else {
                return Err(Error::Encode("error eof while percent decode".to_string()));
            }
        }
        let subs = String::from_utf8(buf.to_vec())?;
        let sub = u8::from_str_radix(subs.as_str(), 16)?;
        ret.push(sub);
    }
    Ok(ret)
}