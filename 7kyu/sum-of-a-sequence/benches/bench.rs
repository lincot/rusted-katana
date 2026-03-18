#![feature(test)]

extern crate test;
use sum_of_a_sequence::sequence_sum;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sequence_sum(black_box(2), black_box(6), black_box(2)));
}
