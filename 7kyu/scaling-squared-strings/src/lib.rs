//! <https://www.codewars.com/kata/56ed20a2c4e5d69155000301/train/rust>

use unchecked::{ExtendFromWithinUnchecked, PushUnchecked};

pub fn scale(s: &str, k: u32, n: u32) -> String {
    const DELIM: char = '\n';

    unsafe fn vertical_scale(res: &mut String, mut chunk_len: usize, n: u32) {
        res.push_unchecked(DELIM);
        chunk_len += DELIM.len_utf8();
        let len = res.len();
        for _ in 0..n - 1 {
            res.as_mut_vec()
                .extend_from_within_unchecked(len - chunk_len..len);
        }
    }

    if s.is_empty() || k == 0 || n == 0 {
        return String::new();
    }
    let mut res = String::with_capacity((s.len() + DELIM.len_utf8()) * k as usize * n as usize);
    let mut chunk_len = 0;
    for c in s.chars() {
        if c == DELIM {
            unsafe { vertical_scale(&mut res, chunk_len, n) };
            chunk_len = 0;
        } else {
            chunk_len += c.len_utf8() * k as usize;
            for _ in 0..k {
                unsafe { res.push_unchecked(c) };
            }
        }
    }
    unsafe { vertical_scale(&mut res, chunk_len, n) };
    res.pop();
    res
}
