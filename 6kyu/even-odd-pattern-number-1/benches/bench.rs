#![feature(test)]

extern crate test;
use even_odd_pattern_number_1::even_odd;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| even_odd(black_box(&[3; 50])));
}
