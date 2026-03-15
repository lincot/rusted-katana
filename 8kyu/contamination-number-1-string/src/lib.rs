//! <https://www.codewars.com/kata/596fba44963025c878000039/train/rust>

pub fn contamination(text: &str, character: &str) -> String {
    character.repeat(text.len())
}
