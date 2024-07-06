#![feature(test)]

extern crate test;
use take_a_number_and_sum_its_digits_raised_to_the_consecutive_powers_and_dot_dot_dot_eureka::sum_dig_pow;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sum_dig_pow(black_box(10), black_box(3_000_000)));
}
