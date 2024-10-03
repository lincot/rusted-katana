//! <https://www.codewars.com/kata/59b7571bbf10a48c75000070/train/rust>

use core::mem::MaybeUninit;
use num_integer::Roots;
use unchecked_std::prelude::*;

pub fn tops(msg: &str) -> String {
    if msg.is_empty() {
        return String::new();
    }

    if msg.is_ascii() {
        return unsafe { String::from_utf8_unchecked(tops_bytes(msg.as_bytes())) };
    }

    let mut gap = 0;
    let capacity = ((8 * msg.len() + 1).sqrt() - 3) / 4 + 1;
    let mut tops = Vec::with_capacity(capacity);
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
    for ch in tops.into_iter().rev() {
        unsafe { res.push_unchecked(ch) };
    }
    res
}

fn tops_bytes(msg: &[u8]) -> Vec<u8> {
    let len = ((8 * msg.len() + 1).sqrt() - 3) / 4 + 1;
    let mut i = 2 * len * len + 3 * len + 1;
    let mut gap = 4 * len;
    let mut tops = Vec::with_capacity(len);
    let mut j = 0;
    while i > 1 {
        i -= gap + 1;
        gap -= 4;
        unsafe {
            *tops.spare_capacity_mut().get_unchecked_mut(j) =
                MaybeUninit::new(*msg.get_unchecked(i));
        }
        j += 1;
    }
    unsafe { tops.set_len(len) };
    tops
}
