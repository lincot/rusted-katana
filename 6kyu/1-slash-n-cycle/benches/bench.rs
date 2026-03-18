#![feature(test)]

extern crate test;
use solution_1_slash_n_cycle::cycle;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| cycle(black_box(197)));
}
