#![feature(test)]

extern crate test;
use closing_in_sum::closing_in_sum;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| closing_in_sum(black_box(8_780_436_960_754_480_978)));
}
