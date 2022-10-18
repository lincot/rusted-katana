//! <https://www.codewars.com/kata/59f11118a5e129e591000134/train/rust>

#![no_std]

extern crate alloc;
use alloc::boxed::Box;

pub fn repeats(arr: &[i32]) -> i32 {
    let mut arr: Box<[_]> = arr.into();

    arr.sort_unstable();

    let mut arr_it = arr.iter();
    let mut prev = arr_it.next().unwrap();

    let mut found = None;
    let mut new_pair = false;

    for x in arr_it {
        if !new_pair && x != prev {
            if let Some(first) = found {
                return first + prev;
            }

            found = Some(prev);
            new_pair = !new_pair;
        }

        prev = x;
        new_pair = !new_pair;
    }

    found.unwrap() + prev
}
