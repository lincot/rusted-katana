#![no_std]
#![feature(test)]

extern crate test;
use jennys_secret_message::greet;
use test::{black_box, Bencher};

#[bench]
fn bench_johny(bencher: &mut Bencher) {
    let input = black_box("Johnny");
    bencher.iter(|| greet(input));
}

#[bench]
fn bench_susan(bencher: &mut Bencher) {
    let input = black_box("Susan");
    bencher.iter(|| greet(input));
}
