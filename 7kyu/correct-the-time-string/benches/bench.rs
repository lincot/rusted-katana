#![no_std]
#![feature(test)]

extern crate test;
use correct_the_time_string::time_correct;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let time_str = black_box("20:40:39");
    bencher.iter(|| time_correct(time_str));
}
