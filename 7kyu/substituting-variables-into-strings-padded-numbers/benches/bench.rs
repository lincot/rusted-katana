#![no_std]
#![feature(test)]

extern crate test;
use substituting_variables_into_strings_padded_numbers::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solution(black_box(5)));
}
