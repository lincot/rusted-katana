//! <https://www.codewars.com/kata/5121303128ef4b495f000001/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use prelude::*;

pub struct Person<'a> {
    pub name: &'a str,
}

impl<'a> Person<'a> {
    pub const fn new(name: &'a str) -> Self {
        Person { name }
    }

    pub fn greet(&self, name: &str) -> String {
        let mut res =
            String::with_capacity("Hello , my name is ".len() + name.len() + self.name.len());
        unsafe {
            res.push_str_unchecked("Hello ");
            res.push_str_unchecked(name);
            res.push_str_unchecked(", my name is ");
            res.push_str_unchecked(self.name);
        }
        res
    }
}
