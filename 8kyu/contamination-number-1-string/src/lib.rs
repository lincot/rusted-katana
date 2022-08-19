//! <https://www.codewars.com/kata/596fba44963025c878000039/train/rust>

use my_prelude::prelude::*;

pub fn contamination(text: &str, character: &str) -> String {
    if character.is_empty() {
        return String::new();
    }
    let mut res = String::with_capacity(text.len());
    for _ in 0..text.len() / character.len() {
        unsafe { res.push_str_unchecked(character) }
    }
    res
}
