#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&[1, 2, 3, 4, 6, 7, 8, 10]);
    bencher.iter(|| solution::all_non_consecutive(arr))
}
