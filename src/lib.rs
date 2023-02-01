#![feature(str_internals)]
#![allow(unused)]

pub mod encoder;
pub mod authority;
pub mod error;
pub mod fragment;
pub mod rpart;
pub mod path;
pub mod query;
pub mod scheme;
pub mod uri;

pub use error::Result;

pub trait Parser: Sized {
    fn decode(s: &str) -> Result<Self>;
    fn encode(&self) -> Result<String>;
}