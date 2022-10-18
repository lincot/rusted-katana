#![no_std]
#![feature(test)]

extern crate test;
use correct_the_time_string::time_correct;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| time_correct(black_box("20:40:39")));
}
