use std::fmt::format;
use crate::encoder::Decoder;
use crate::Parser;
use super::authority::Authority;
use super::path::Path;
use super::error::Result;

#[derive(Debug)]
pub struct RPart {
    pub authority: Option<Authority>,
    pub path: Option<Path>,
}

impl RPart {
    pub fn new() -> Self {
        RPart {
            authority: None,
            path: None,
        }
    }
}

impl Parser for RPart {
    fn decode(s: &str) -> Result<Self> {
        let mut rpart = RPart::new();
        let mut path = String::new();
        let mut author = String::new();
        if s.starts_with("//") {
            author = s.trim_start_matches("//").to_string();
            let auth_end = author.find("/");
            if let Some(index) = auth_end {
                let t = author.clone();
                let (a, p) = t.split_at(index);
                author = a.to_string();
                path = p.to_string();
                path = format!("{}", path);
            }
        } else {
            path = s.to_string()
        }
        if author.len() > 0 {
            let auth_r = Authority::decode(author.as_str())?;
            rpart.authority = Some(auth_r)
        }
        if path.len() > 0 {
            let path_r = Path::decode(path.as_str())?;
            rpart.path = Some(path_r)
        }
        Ok(rpart)
    }

    fn encode(&self) -> Result<String> {
        let mut buf = String::new();
        if let Some(auth) = &self.authority {
            let r = auth.encode()?;
            buf = format!("//{}", r);
        }
        if let Some(path) = &self.path {
            let r = path.encode()?;
            buf.push_str(r.as_str())
        }
        Ok(buf)
    }
}