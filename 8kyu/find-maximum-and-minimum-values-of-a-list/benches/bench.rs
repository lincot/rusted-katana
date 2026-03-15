#![feature(test)]

extern crate test;
use core::array;
use find_maximum_and_minimum_values_of_a_list::{maximum, minimum};
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut arr: [_; if cfg!(miri) { 16 } else { 1024 }] = array::from_fn(|i| 500 + i as i32);
    arr[arr.len() / 2] = 2;
    arr[arr.len() / 2 + 1] = 1_000_000;
    bencher.iter(|| (maximum(black_box(&arr)), minimum(black_box(&arr))));
}
