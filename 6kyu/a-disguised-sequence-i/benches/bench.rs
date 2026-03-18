#![feature(test)]

extern crate test;
use a_disguised_sequence_i::fcn;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| fcn(black_box(21)));
}
