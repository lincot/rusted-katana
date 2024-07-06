//! <https://www.codewars.com/kata/59b7571bbf10a48c75000070/train/rust>

use num_integer::Roots;
use unchecked_std::prelude::*;

pub fn tops(msg: &str) -> String {
    if msg.is_empty() {
        return String::new();
    }

    let mut gap = 0;
    let mut tops = Vec::with_capacity(((8 * msg.len() + 1).sqrt() - 3) / 4 + 1);
    let mut msg = msg.chars();
    msg.next();
    loop {
        for _ in 0..gap {
            msg.next();
        }
        if let Some(next) = msg.next() {
            unsafe { tops.push_unchecked(next) };
        } else {
            break;
        }
        gap += 4;
    }

    let mut res = String::with_capacity(4 * tops.len());
    for c in tops.into_iter().rev() {
        unsafe { res.push_unchecked(c) };
    }
    res
}
