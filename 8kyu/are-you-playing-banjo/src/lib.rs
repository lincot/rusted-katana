//! <https://www.codewars.com/kata/53af2b8861023f1d88000832/train/rust>

use my_prelude::prelude::*;

pub fn are_you_playing_banjo(name: &str) -> String {
    let output = |text: &str| {
        let mut res = String::with_capacity(name.len() + text.len());
        unsafe {
            res.push_str_unchecked(name);
            res.push_str_unchecked(text);
        }
        res
    };

    if [b'R', b'r'].contains(&name.bytes().next().unwrap()) {
        output(" plays banjo")
    } else {
        output(" does not play banjo")
    }
}
