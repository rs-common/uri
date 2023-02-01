use std::str::FromStr;
use super::encoder::{Decoder, Encoder};
use super::error::{Error, Result};
use super::Parser;


#[derive(Debug, Clone)]
pub struct Scheme(String);

impl Parser for Scheme {
    fn decode(s: &str) -> Result<Self> {
        if !s.as_bytes()[0].is_ascii_alphabetic() {
            return Err(Error::Encode("scheme must start with alphabetic".to_string()));
        }
        let mut dec = Decoder::new(s);
        dec.allowed().set_alphanum().set(vec![b'+', b'-', b'.']);
        let rst = dec.decode()?;
        Ok(Scheme(rst))
    }

    fn encode(&self) -> Result<String> {
        if !self.0.as_bytes()[0].is_ascii_alphabetic() {
            return Err(Error::Encode("scheme must start with alphabetic".to_string()));
        }
        let mut enc = Encoder::new(self.0.as_str());
        enc.allowed().set_alphanum().set(vec![b'+', b'-', b'.']);
        let rst = enc.encode()?;
        Ok(rst)
    }
}