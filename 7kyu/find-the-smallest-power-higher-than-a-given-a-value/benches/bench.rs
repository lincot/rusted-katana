#![no_std]
#![feature(test)]

extern crate test;
use find_the_smallest_power_higher_than_a_given_a_value::find_next_power;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_next_power(black_box(33_610_383), black_box(5)));
}
