#![feature(test)]

extern crate test;
use sum_two_arrays::add_arrays;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr_a = black_box(&[4, 7, 3]);
    let arr_b = black_box(&[1, 2, 3]);
    bencher.iter(|| add_arrays(arr_a, arr_b));
}
