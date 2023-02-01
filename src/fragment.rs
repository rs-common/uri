use super::encoder::{Decoder, Encoder};
use super::Parser;
use super::error::Result;

#[derive(Debug)]
pub struct Fragment(String);

impl Parser for Fragment {
    fn decode(s: &str) -> crate::Result<Self> {
        let mut dec = Decoder::new(s);
        dec.set_decode_pct();
        dec.allowed().set_unreserved().set_subdelims().set(vec![b'/', b'?']);
        let r = dec.decode()?;
        Ok(Fragment(r))
    }

    fn encode(&self) -> crate::Result<String> {
        let mut enc = Encoder::new(self.0.as_str());
        enc.set_encode_pct();
        enc.allowed().set_unreserved().set_subdelims().set(vec![b'/', b'?']);
        let r = enc.encode()?;
        Ok(r)
    }
}