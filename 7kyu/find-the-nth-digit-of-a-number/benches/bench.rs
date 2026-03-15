#![feature(test)]

extern crate test;
use find_the_nth_digit_of_a_number::find_digit;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_digit(black_box(-2825), black_box(3)));
}
