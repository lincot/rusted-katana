//! <https://www.codewars.com/kata/551f37452ff852b7bd000139/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use lexical::{to_string_with_options, NumberFormatBuilder, WriteIntegerOptions};

pub fn add_binary(a: u64, b: u64) -> String {
    const FORMAT: u128 = NumberFormatBuilder::binary();
    to_string_with_options::<_, FORMAT>(a + b, &WriteIntegerOptions::new())
}
