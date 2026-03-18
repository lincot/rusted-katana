#![feature(test)]

extern crate test;
use function_2_squaring_an_argument::square;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| square(black_box(50)));
}
