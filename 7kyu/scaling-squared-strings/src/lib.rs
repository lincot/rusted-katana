//! <https://www.codewars.com/kata/56ed20a2c4e5d69155000301/train/rust>

use my_prelude::prelude::*;

pub fn scale(s: &str, k: u32, n: u32) -> String {
    if s.is_empty() || k == 0 || n == 0 {
        return String::new();
    }
    let s = s.as_bytes();
    let substring_len = s.iter().position(|&b| b == b'\n').unwrap_or(s.len());
    let substrings_count = (s.len() + 1) / (substring_len + 1);
    assert!(s.len() == substrings_count * substring_len + substrings_count - 1);
    let chunk_len = substring_len * k as usize + 1;
    let mut res = Vec::with_capacity(chunk_len * substrings_count * n as usize);
    let mut i = 0;
    for _ in 0..substrings_count {
        for _ in 0..substring_len {
            assert!(unsafe { s.get_unchecked(i) }.is_ascii());
            for _ in 0..k {
                unsafe { res.push_unchecked(*s.get_unchecked(i)) };
            }
            i += 1;
        }
        unsafe { res.push_unchecked(b'\n') };
        for _ in 0..n - 1 {
            unsafe { res.extend_from_within_unchecked(res.len() - chunk_len..res.len()) };
        }
        i += 1;
    }
    res.pop();
    unsafe { String::from_utf8_unchecked(res) }
}
