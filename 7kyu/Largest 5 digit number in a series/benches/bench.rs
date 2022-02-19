#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const NUM: &str = "4566002453929736054350547426096148205514935609611179318220188715941036112872495043452910538358";

#[bench]
fn bench(bencher: &mut Bencher) {
    let num = black_box(NUM);

    bencher.iter(|| solution::largest_five_digit_number(num))
}
