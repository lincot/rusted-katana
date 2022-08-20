//! <https://www.codewars.com/kata/55225023e1be1ec8bc000390/train/rust>

use my_prelude::prelude::*;

pub fn greet(input: &str) -> String {
    if input == "Johnny" {
        "Hello, my love!".into()
    } else {
        let mut res = String::with_capacity("Hello, !".len() + input.len());
        unsafe {
            res.push_str_unchecked("Hello, ");
            res.push_str_unchecked(input);
            res.push_unchecked('!');
        }
        res
    }
}
