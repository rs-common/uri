use std::fs;
use crate::encoder::{Decoder, Encoder};
use crate::Parser;
use super::error::{Error, Result};

#[derive(Debug)]
pub struct Path(String);

// todo: 增加消除点段
impl Path {
    pub fn new(s: &str) -> Self {
        Path(s.to_string())
    }
}

impl Parser for Path {
    fn decode(s: &str) -> Result<Self> {
        let mut buf: Vec<String> = Vec::new();
        for sub in s.split('/') {
            let mut dec = Decoder::new(sub);
            dec.allow_empty();
            dec.set_decode_pct();
            dec.allowed().set_unreserved().set_subdelims().set([b':', b'@'].to_vec());
            let r = dec.decode()?;
            buf.push(r)
        }
        let r = buf.join("/");
        Ok(Path(r))
    }

    fn encode(&self) -> Result<String> {
        let mut buf: Vec<String> = Vec::new();
        for sub in self.0.split('/') {
            let mut enc = Encoder::new(sub);
            enc.allow_empty();
            enc.set_encode_pct();
            enc.allowed().set_unreserved().set_subdelims().set([b':', b'@'].to_vec());
            let r = enc.encode()?;
            buf.push(r)
        }
        let r = buf.join("/");
        Ok(r)
    }
}

#[test]
fn parse() {
    let s = "/a/b/";
    println!("{}", s.len());
    let a: Vec<&str> = s.split('/').collect();
    println!("{:#?}", a)
}

#[test]
fn decode() {
    let r = Path::decode("a%E4%BB%A3/b/c").unwrap();
    println!("{:?}", r);
}

#[test]
fn encode() {
    let s = Path("a代/b/c".to_string());
    println!("{:?}", s.encode())
}