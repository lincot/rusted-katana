//! <https://www.codewars.com/kata/52fba66badcd10859f00097e/train/rust>

use unchecked_core::PushUnchecked;

pub fn disemvowel(s: &str) -> String {
    let mut res = String::with_capacity(s.len());
    for b in s.bytes() {
        if !b"eaiouEAIOU".contains(&b) {
            unsafe { res.as_mut_vec().push_unchecked(b) };
        }
    }
    res
}
