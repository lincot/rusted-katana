#![feature(test)]

extern crate test;
use sum_of_digits_digital_root::digital_root;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| digital_root(black_box(999_493_193)));
}
