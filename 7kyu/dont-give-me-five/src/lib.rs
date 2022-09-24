//! <https://www.codewars.com/kata/5813d19765d81c592200001a/train/rust>

#![no_std]

pub const fn dont_give_me_five(start: isize, end: isize) -> isize {
    if start * end >= 0 {
        if start >= 0 {
            dont_give_me_five_unary(end) - dont_give_me_five_unary(start - 1)
        } else {
            dont_give_me_five_unary(-start) - dont_give_me_five_unary(-end - 1)
        }
    } else {
        dont_give_me_five_unary(end) + dont_give_me_five_unary(-start) + 1
    }
}

const fn dont_give_me_five_unary(mut end: isize) -> isize {
    let mut res = 0;
    let mut c = 1;

    while end != 0 {
        let mut d = end % 10;

        if d > 5 {
            d -= 1;
        } else if d == 5 {
            res = -1;
        }
        res += c * d;

        c *= 9;
        end /= 10;
    }

    res
}
