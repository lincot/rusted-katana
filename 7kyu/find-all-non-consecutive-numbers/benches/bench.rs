#![feature(test)]

extern crate test;
use find_all_non_consecutive_numbers::all_non_consecutive;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&[1, 2, 3, 4, 6, 7, 8, 10]);
    bencher.iter(|| all_non_consecutive(arr));
}
