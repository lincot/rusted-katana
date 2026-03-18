#![feature(test)]

extern crate test;
use suitcase_packing::fit_in;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| fit_in(black_box(1), black_box(2), black_box(3), black_box(2)));
}
