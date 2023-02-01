use std::collections::{HashMap, HashSet};
use std::iter::Map;
use crate::error::Error::Path;
use super::encoder::{Encoder, Decoder};
use super::Parser;
use super::error::{Result, Error};

type QData = HashMap<String, HashSet<String>>;

#[derive(Debug)]
pub struct Query(QData);

impl Query {
    pub fn new() -> Self {
        Query(HashMap::new())
    }

    pub fn set<T: Into<String>>(&mut self, key: String, val: T) -> &mut Self {
        insert(&mut self.0, key, val);
        self
    }

    pub fn set_str(&mut self, s: &str) -> Result<&mut Self> {
        let (k, v) = parse_str(s)?;
        insert(&mut self.0, k, v);
        Ok(self)
    }
}

impl Parser for Query {
    fn decode(s: &str) -> Result<Self> {
        let mut mp = HashMap::<String, HashSet<String>>::new();
        let mut dec = Decoder::new(s);
        dec.set_decode_pct();
        dec.allowed().set_unreserved().set_subdelims().set([b':', b'@', b'/', b'?'].to_vec());
        let qs = dec.decode()?;
        for v in qs.split("&") {
            let (k, v) = parse_str(v)?;
            insert(&mut mp, k, v);
        }
        Ok(Query(mp))
    }

    fn encode(&self) -> Result<String> {
        let mut buf = Vec::<String>::new();
        let mp = &self.0;
        for (k, vl) in mp.iter() {
            for v in vl {
                buf.push(format!("{}={}", k, v))
            }
        }
        let s = buf.join("&");
        let mut enc = Encoder::new(s.as_str());
        enc.set_encode_pct();
        enc.allowed().set_unreserved().set_subdelims().set([b':', b'@', b'/', b'?'].to_vec());
        let r = enc.encode()?;
        Ok(s)
    }
}

fn parse_str(s: &str) -> Result<(String, String)> {
    let a: Vec<String> = s.split("=").map(|s| s.to_string()).collect();
    if a.len() != 2 {
        return Err(Error::Path("invalid path item string".to_string()));
    }
    Ok((a[0].clone(), a[1].clone()))
}

fn insert<T: Into<String>>(q: &mut QData, k: String, v: T) -> &mut QData {
    match q.get_mut(k.as_str()) {
        None => {
            let mut set = HashSet::new();
            set.insert(v.into());
            q.insert(k, set);
        }
        Some(set) => {
            set.insert(v.into());
        }
    }
    q
}

#[test]
fn path_decode() {
    let s = "version=1.0&qlist=10&qlist=20";
    let r = Query::decode(s);
    match r {
        Ok(q) => println!("{:?}", q),
        Err(e) => println!("{}", e),
    }
}

#[test]
fn path_encode() {
    let mut q = Query::new();
    q.set("d".into(), 10.to_string());
    q.set("d".into(), 20.to_string());
    let r  = q.encode();
    match r {
        Ok(q) => println!("{:?}", q),
        Err(e) => println!("{}", e),
    }
}