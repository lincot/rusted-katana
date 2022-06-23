//! <https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust>

pub fn disemvowel(s: &str) -> String {
    unsafe {
        String::from_utf8_unchecked(s.bytes().filter(|b| !b"eaiouEAIOU".contains(b)).collect())
    }
}
