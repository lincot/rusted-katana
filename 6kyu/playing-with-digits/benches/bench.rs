#![feature(test)]

extern crate test;
use playing_with_digits::dig_pow;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| dig_pow(black_box(46288), black_box(3)));
}
