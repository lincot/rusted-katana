//! <https://www.codewars.com/kata/598d91785d4ce3ec4f000018/train/rust>

#![no_std]

extern crate alloc;
use alloc::vec::Vec;

pub fn word_value(words: &[&str]) -> Vec<i32> {
    let mut res = Vec::with_capacity(words.len());
    unsafe { res.set_len(words.len()) };
    let mut res_ptr = res.as_mut_ptr();
    let mut i = 1;
    for word in words {
        unsafe {
            *res_ptr = 0;
            for b in word.bytes() {
                *res_ptr += if b.is_ascii_lowercase() {
                    (b - b'a' + 1) as _
                } else {
                    0
                };
            }
            *res_ptr *= i;
            res_ptr = res_ptr.add(1);
        }
        i += 1;
    }
    res
}
