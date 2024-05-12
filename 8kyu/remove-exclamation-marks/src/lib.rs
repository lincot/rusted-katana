//! <https://www.codewars.com/kata/57a0885cbb9944e24c00008e/train/rust>

use unchecked_core::PushUnchecked;

pub fn remove_exclamation_marks(input: &str) -> String {
    let mut res = Vec::with_capacity(input.len());
    for b in input.bytes() {
        if b != b'!' {
            unsafe { res.push_unchecked(b) };
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
