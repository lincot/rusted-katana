//! <https://www.codewars.com/kata/61123a6f2446320021db987d/train/rust>

#![no_std]

pub const fn prev_mult_of_three(mut n: i32) -> i32 {
    while n % 3 != 0 {
        n /= 10;
    }

    if n == 0 {
        -1
    } else {
        n
    }
}
