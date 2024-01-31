//! <https://www.codewars.com/kata/587731fda577b3d1b0001196/train/rust>

use unchecked::{ExtendUnchecked, PushUnchecked};

pub fn camel_case(str: &str) -> String {
    let mut res = String::with_capacity(2 * str.len() + str.len() / 3);

    let mut chars = str.chars();

    for c in chars.by_ref() {
        if c != ' ' {
            unsafe { res.extend_unchecked(c.to_uppercase()) };
            break;
        }
    }
    while let Some(c) = chars.next() {
        if c == ' ' {
            for c in chars.by_ref() {
                if c != ' ' {
                    unsafe { res.extend_unchecked(c.to_uppercase()) };
                    break;
                }
            }
        } else {
            unsafe { res.push_unchecked(c) };
        }
    }

    res
}
