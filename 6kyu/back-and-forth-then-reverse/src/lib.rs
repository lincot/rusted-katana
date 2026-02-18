//! <https://www.codewars.com/kata/60cc93db4ab0ae0026761232/train/rust>

use unchecked_std::prelude::*;

pub fn arrange(s: &[i32]) -> Vec<i32> {
    let mut res = Vec::with_capacity(s.len());
    if s.is_empty() {
        return res;
    }

    let mut head = 0;
    let mut tail = s.len() - 1;
    while head < tail {
        unsafe {
            res.push_unchecked(*s.get_unchecked(head));
            res.push_unchecked(*s.get_unchecked(tail));
            head += 1;
            tail -= 1;
            if head >= tail {
                break;
            }
            res.push_unchecked(*s.get_unchecked(tail));
            res.push_unchecked(*s.get_unchecked(head));
            head += 1;
            tail -= 1;
        }
    }
    if head == tail {
        unsafe { res.push_unchecked(*s.get_unchecked(head)) };
    }
    res
}
