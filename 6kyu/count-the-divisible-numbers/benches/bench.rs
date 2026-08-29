#![feature(test)]

extern crate test;

use count_the_divisible_numbers::divisible_count;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| divisible_count(black_box(11), black_box(345), black_box(17)));
}
