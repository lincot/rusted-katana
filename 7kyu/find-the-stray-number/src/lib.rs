//! <https://www.codewars.com/kata/57f609022f4d534f05000024/train/rust>

#![no_std]

pub fn stray(arr: &[u32]) -> u32 {
    let &[first, second, third, ..] = arr else {
        panic!();
    };
    if first == second {
        *arr[2..].iter().find(|&&x| x != first).unwrap()
    } else if first == third {
        second
    } else {
        first
    }
}
