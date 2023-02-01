use crate::Parser;
use super::fragment::Fragment;
use super::query::Query;
use super::rpart::RPart;
use super::scheme::Scheme;
use super::error::{Result, Error};

#[derive(Debug)]
pub struct URI {
    pub scheme: Option<Scheme>,
    pub rpart: Option<RPart>,
    pub query: Option<Query>,
    pub fragment: Option<Fragment>,
}

impl URI {
    pub fn new() -> Self {
        URI {
            scheme: None,
            rpart: None,
            query: None,
            fragment: None,
        }
    }

    fn step_decode(&mut self, step: u8, buf: &mut Vec<u8>) -> Result<&mut Self> {
        let s = String::from_utf8(buf.to_vec())?;
        let s = s.as_str();
        match step {
            0 => {
                let scheme = Scheme::decode(s)?;
                self.scheme = Some(scheme)
            }
            1 => {
                let rpart = RPart::decode(s)?;
                self.rpart = Some(rpart)
            }
            2 => {
                let query = Query::decode(s)?;
                self.query = Some(query)
            }
            3 => {
                let fragm = Fragment::decode(s)?;
                self.fragment = Some(fragm)
            }
            _ => return Err(Error::Decode("unknown decode step".to_string()))
        }
        buf.clear();
        Ok(self)
    }
}

impl Parser for URI {
    fn decode(s: &str) -> crate::Result<Self> {
        let mut r = URI::new();
        let mut s_iter = s.as_bytes().iter();
        let mut buf = Vec::<u8>::new();
        let mut step = 0u8; // 0 = scheme, 1 = rpart, 2 = query, 3 = fragment
        loop {
            let iter_r = s_iter.next();
            match (iter_r, step) {
                (Some(b':'), 0) => {
                    r.step_decode(step, &mut buf);
                    step = 1
                }
                (Some(b'/'), 0) => {
                    buf.push(b'/');
                    step = 1
                }
                (Some(b'?'), 0 | 1) => {
                    r.step_decode(1, &mut buf);
                    step = 2
                }
                (Some(b'#'), 0..=2) => {
                    r.step_decode(step, &mut buf);
                    step = 3
                }
                (None, _) => {
                    if buf.len() > 0 {
                        if step == 0 {
                            step = 1;
                        }
                        r.step_decode(step, &mut buf)?;
                    }
                    break;
                }
                (Some(item), _) => {
                    buf.push(*item)
                }
            }
        }
        Ok(r)
    }

    fn encode(&self) -> Result<String> {
        let mut buf = String::new();
        if let Some(sch) = &self.scheme {
            let r = sch.encode()?;
            buf.push_str(format!("{}:", r).as_str())
        };
        if let Some(rpt) = &self.rpart {
            let r = rpt.encode()?;
            buf.push_str(r.as_str())
        }
        if let Some(query) = &self.query {
            let r = query.encode()?;
            buf.push_str(format!("?{}", r).as_str())
        }
        if let Some(frag) = &self.fragment {
            let r = frag.encode()?;
            buf.push_str(format!("#{}", r).as_str());
        }
        Ok(buf)
    }
}
