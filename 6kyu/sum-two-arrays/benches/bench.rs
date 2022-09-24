#![no_std]
#![feature(test)]

extern crate test;
use sum_two_arrays::add_arrays;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr_a = black_box(&[3, 2, 6, 6, 1]);
    let arr_b = black_box(&[-7, 2, 2, 8, 5]);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(add_arrays(arr_a, arr_b));
        }
    });
}
