#![no_std]
#![feature(test)]

extern crate test;
use largest_5_digit_number_in_a_series::largest_five_digit_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| largest_five_digit_number(black_box("4566002453929736054350547426096148205514935609611179318220188715941036112872495043452910538358")));
}
