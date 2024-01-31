#![feature(test)]

extern crate test;
use solution_8kyu_interpreters_hq9_plus::hq9;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| hq9(black_box("9")));
}
