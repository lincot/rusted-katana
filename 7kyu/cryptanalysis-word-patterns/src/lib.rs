//! <https://www.codewars.com/kata/5f3142b3a28d9b002ef58f5e/train/rust>

use unchecked_core::PushUnchecked;

pub fn word_pattern(word: &str) -> String {
    let mut encountered = heapless::Vec::<_, 52>::new();
    let mut res = String::with_capacity(3 * word.len());
    let mut word = word.as_bytes().iter();
    if let Some(&b) = word.next() {
        let b = if b < b'a' { b + b'a' - b'A' } else { b };
        assert!(b.is_ascii_lowercase());
        unsafe {
            encountered.push_unchecked(b);
            res.as_mut_vec().push_unchecked(b'0');
        }
    }
    for &b in word {
        let b = if b < b'a' { b + b'a' - b'A' } else { b };
        assert!(b.is_ascii_lowercase());
        let pos = encountered
            .iter()
            .position(|&b_| b_ == b)
            .unwrap_or_else(|| {
                unsafe { encountered.push_unchecked(b) };
                encountered.len() - 1
            }) as u8;
        unsafe {
            res.push_unchecked('.');
            if pos >= 10 {
                res.as_mut_vec().push_unchecked(pos / 10 + b'0');
            }
            res.as_mut_vec().push_unchecked(pos % 10 + b'0');
        }
    }
    res
}
