#![feature(test)]

extern crate test;
use core::array;
use reach_me_and_sum_my_digits::sum_dig_nth_term;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let pattern: [_; 100] = array::from_fn(|i| i as u32);
    bencher.iter(|| sum_dig_nth_term(black_box(10), black_box(&pattern), black_box(350)));
}
