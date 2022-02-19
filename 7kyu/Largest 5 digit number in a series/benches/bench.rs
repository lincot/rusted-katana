#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let num = black_box("4566002453929736054350547426096148205514935609611179318220188715941036112872495043452910538358");
    bencher.iter(|| solution::largest_five_digit_number(num))
}
