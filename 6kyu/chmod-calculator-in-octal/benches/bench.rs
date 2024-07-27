#![feature(test)]

extern crate test;
use chmod_calculator_in_octal::chmod_calculator;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let perm = [("user", "r-x"), ("group", "r-x"), ("other", "---")].into();
    bencher.iter(|| chmod_calculator(black_box(&perm)));
}
