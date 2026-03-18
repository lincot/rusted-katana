#![feature(test)]

extern crate test;
use largest_integer_exponent::get_exponent;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_exponent(black_box(-250), black_box(5)));
}
