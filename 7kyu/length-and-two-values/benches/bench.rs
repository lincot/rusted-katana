#![no_std]
#![feature(test)]

extern crate test;
use length_and_two_values::alternate;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(10000);
    let first_value = black_box("blue");
    let second_value = black_box("red");
    bencher.iter(|| alternate(n, first_value, second_value));
}
