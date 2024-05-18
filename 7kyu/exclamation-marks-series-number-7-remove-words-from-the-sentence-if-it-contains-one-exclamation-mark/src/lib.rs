//! <https://www.codewars.com/kata/57fafb6d2b5314c839000195/train/rust>

use unchecked_std::prelude::*;

pub fn remove(s: &str) -> String {
    let s = s.as_bytes();
    let mut res = Vec::with_capacity(s.len());
    let mut prev_space = usize::MAX;
    let mut excl_count = 0;
    let mut first = true;
    for (i, &b) in s.iter().enumerate() {
        if b == b' ' {
            if excl_count != 1 {
                unsafe {
                    if !first {
                        res.push_unchecked(b' ');
                    }
                    res.extend_from_slice_unchecked(s.get_unchecked(prev_space.wrapping_add(1)..i));
                }
                first = false;
            }
            prev_space = i;
            excl_count = 0;
        } else if b == b'!' {
            excl_count += 1;
        }
    }
    if excl_count != 1 {
        unsafe {
            if !first {
                res.push_unchecked(b' ');
            }
            res.extend_from_slice_unchecked(s.get_unchecked(prev_space.wrapping_add(1)..));
        }
    }
    unsafe { String::from_utf8_unchecked(res) }
}
