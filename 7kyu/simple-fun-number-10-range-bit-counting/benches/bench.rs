#![no_std]
#![feature(test)]

extern crate test;
use simple_fun_number_10_range_bit_counting::range_bit_count;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(10);
    let b = black_box(100);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(range_bit_count(a, b));
        }
    });
}
