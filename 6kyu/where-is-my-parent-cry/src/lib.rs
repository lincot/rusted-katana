//! <https://www.codewars.com/kata/58539230879867a8cd00011c/train/rust>

use core::hint::unreachable_unchecked;
use unchecked::PushUnchecked;

pub fn find_children(dancing_brigade: &str) -> String {
    let mut res = String::with_capacity(dancing_brigade.len());

    let mut family_sizes = [0usize; 26];
    for b in dancing_brigade.bytes() {
        if b.is_ascii_lowercase() {
            family_sizes[(b - b'a') as usize] += 1;
        } else if b.is_ascii_uppercase() {
            family_sizes[(b - b'A') as usize] += 1;
        }
    }

    for (i, family_size) in (0..).zip(family_sizes).filter(|&(_, x)| x != 0) {
        unsafe {
            res.as_mut_vec().push_unchecked(i + b'A');
            for _ in 0..family_size - 1 {
                res.as_mut_vec().push_unchecked(i + b'a');
            }
        }
    }

    if res.len() != dancing_brigade.len() {
        unsafe { unreachable_unchecked() };
    }
    res
}
