#![feature(test)]

extern crate test;
use core::array;
use find_the_smallest_integer_in_the_array::find_smallest_int;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut arr: [_; if cfg!(miri) { 16 } else { 32 }] = array::from_fn(|i| 500 + i as i32);
    arr[arr.len() / 2] = 5;
    bencher.iter(|| find_smallest_int(black_box(&arr)));
}
