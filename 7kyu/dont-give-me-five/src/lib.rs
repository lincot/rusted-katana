//! <https://www.codewars.com/kata/5813d19765d81c592200001a/train/rust>

use core::cmp::Ordering;
use digital::Next2Digits;

pub fn dont_give_me_five(start: isize, end: isize) -> isize {
    if start * end >= 0 {
        if start >= 0 {
            dont_give_me_five_unary(end)
                - if start == 0 {
                    -1
                } else {
                    dont_give_me_five_unary(start - 1)
                }
        } else {
            dont_give_me_five_unary(-start)
                - if end == 0 {
                    -1
                } else {
                    dont_give_me_five_unary(-end - 1)
                }
        }
    } else {
        dont_give_me_five_unary(end) + dont_give_me_five_unary(-start) + 1
    }
}

fn dont_give_me_five_unary(end: isize) -> isize {
    let mut end = end as usize;

    let mut res = 0;
    let mut c = 1;

    while let Some(digits) = end.next_2_digits(true) {
        for &(mut d) in digits.iter().rev() {
            match d.cmp(&5) {
                Ordering::Greater => d -= 1,
                Ordering::Equal => res = -1,
                Ordering::Less => {}
            }
            res += c * d as isize;

            c *= 9;
        }
    }

    if end != 0 {
        match end.cmp(&5) {
            Ordering::Greater => end -= 1,
            Ordering::Equal => res = -1,
            Ordering::Less => {}
        }
        res += c * end as isize;
    }

    res
}
