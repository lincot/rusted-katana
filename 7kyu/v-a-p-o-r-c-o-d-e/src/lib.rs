//! <https://www.codewars.com/kata/5966eeb31b229e44eb00007a/train/rust>

use unchecked::{ExtendUnchecked, PushStrUnchecked, PushUnchecked};

pub fn vaporcode(s: &str) -> String {
    let mut res = String::with_capacity(3 * s.len());
    let mut not_first = false;
    for c in s.chars() {
        if c != ' ' {
            unsafe {
                if not_first {
                    res.push_str_unchecked("  ");
                } else {
                    not_first = true;
                }
                if c.is_lowercase() {
                    res.extend_unchecked(c.to_uppercase());
                } else {
                    res.push_unchecked(c);
                }
            }
        }
    }
    res
}
