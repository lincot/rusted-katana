//! <https://www.codewars.com/kata/5748a883eb737cab000022a6/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use hashbrown::HashMap;

pub fn cannons_ready(gunners: HashMap<&str, &str>) -> String {
    if gunners.values().all(|s| s.bytes().next() == Some(b'a')) {
        "Fire!".into()
    } else {
        "Shiver me timbers!".into()
    }
}
