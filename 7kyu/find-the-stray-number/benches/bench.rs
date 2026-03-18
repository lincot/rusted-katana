#![feature(test)]

extern crate test;
use core::array;
use find_the_stray_number::stray;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut arr: [_; if cfg!(miri) { 15 } else { 1023 }] = array::from_fn(|_| 1);
    arr[arr.len() / 2] = 2;
    bencher.iter(|| stray(black_box(&arr)));
}
