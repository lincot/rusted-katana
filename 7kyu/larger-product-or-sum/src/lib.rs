//! <https://www.codewars.com/kata/5c4cb8fc3cf185147a5bdd02/train/rust>

#![no_std]

extern crate alloc;
use alloc::string::String;
use core::cmp::Ordering;

pub fn sum_or_product(list: &[i64], n: usize) -> String {
    let mut list = list.to_vec();
    if list.len() < 512 {
        list.sort_unstable();
    } else {
        radsort::sort(&mut list);
    }

    let sum = list.iter().rev().take(n).sum::<i64>();
    let product = list.iter().take(n).product();

    match sum.cmp(&product) {
        Ordering::Greater => "sum".into(),
        Ordering::Less => "product".into(),
        Ordering::Equal => "same".into(),
    }
}
