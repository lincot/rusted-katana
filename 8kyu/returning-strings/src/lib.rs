//! <https://www.codewars.com/kata/55a70521798b14d4750000a4/train/rust>

use unchecked_std::prelude::*;

pub fn greet(name: &str) -> String {
    let mut res = String::with_capacity("Hello,  how are you doing today?".len() + name.len());
    unsafe {
        res.push_str_unchecked("Hello, ");
        res.push_str_unchecked(name);
        res.push_str_unchecked(" how are you doing today?");
    }
    res
}
