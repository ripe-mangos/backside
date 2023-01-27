#![no_std]

#![doc = include_str!("../README.docs-rs.md")]

//!
//! ```text
//! /|               /|        <>   /|
//! ||_   __     ___ ||    .// |\   ||  __        /|//.  .//
//! | \\ // \\  / // ||// //.  || // | /_\\       ||    //.
//! \_// \\_/\\ \_\\ \|\\ .//  \| \\_/ \__. ::<_> |/    .//
//! ```
//!

extern crate alloc;

extern crate backside_parser;

pub(crate) use alloc::{vec::Vec, string::{String, ToString}};

mod error;

pub use error::{Error, Result};
//pub use style::Style;

pub struct Script {
    buf: String,
    info: Vec<(String, String)>,
  //  styles: Vec<Style>,
}
impl Script {
    pub fn from_str(s: &str) -> Self {
        Self {
            buf: String::from(s),
            info: Vec::new(),
 //           styles: Vec::new(),
        }
    }
    pub fn init(&self) {
        backside_parser::parse_sections(self.buf.as_str());
    }
}

#[macro_export]
macro_rules! init_helper {
    () => {
        macro_rules! parse {
            () => {
                match v.parse::<i8>().unwrap() {
                    -1 => true,
                     0 => false,
                    _  => unimplemented!(),
                }
            };
            ($type:ident) => {
                v.parse::<$type>().unwrap()
            };
        }
    };
}
