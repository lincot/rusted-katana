#![no_std]
#![feature(test)]

extern crate test;
use convert_a_boolean_to_a_string::boolean_to_string;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let b = black_box(true);
    bencher.iter(|| {
        for _ in 0..1_000_000 {
            black_box(boolean_to_string(b));
        }
    });
}
