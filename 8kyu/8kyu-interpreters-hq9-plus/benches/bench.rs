#![no_std]
#![feature(test)]

extern crate test;
use solution_8kyu_interpreters_hq9_plus::hq9;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let code = black_box("9");
    bencher.iter(|| hq9(code));
}
