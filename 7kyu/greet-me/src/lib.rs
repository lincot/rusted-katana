//! <https://www.codewars.com/kata/535474308bb336c9980006f2/train/rust>

use unchecked_core::{ExtendUnchecked, PushStrUnchecked, PushUnchecked};

pub fn greet(name: &str) -> String {
    let mut res = String::with_capacity("Hello !".len() + 4 + name.len() + name.len() / 2);
    unsafe {
        res.push_str_unchecked("Hello ");
        let mut name = name.chars();
        if let Some(x) = name.next() {
            res.extend_unchecked(x.to_uppercase());
        }
        for x in name {
            res.extend_unchecked(x.to_lowercase());
        }
        res.push_unchecked('!');
    }
    res
}
