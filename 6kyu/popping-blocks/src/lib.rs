//! <https://www.codewars.com/kata/651bfcbcdb0e8b104175b97e/train/rust>

use unchecked_std::prelude::*;

pub fn pop_blocks(lst: &[String]) -> Vec<String> {
    let mut res = Vec::with_capacity(lst.len());
    let mut remove_last = false;
    for s in lst {
        loop {
            if res.last() == Some(&s) {
                remove_last = true;
            } else {
                if remove_last {
                    res.pop();
                    remove_last = false;
                    continue;
                }
                unsafe { res.push_unchecked(s) };
            }
            break;
        }
    }
    if remove_last {
        res.pop();
    }
    res.into_iter().map(Into::into).collect()
}
