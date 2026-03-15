#![feature(test)]

extern crate test;
use simple_multiplication::simple_multiplication;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| simple_multiplication(black_box(u8::MAX / 9)));
}
