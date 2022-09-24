#![no_std]
#![feature(test)]

extern crate test;
use simple_letter_removal::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("abracadabra");
    let k = black_box(6);
    bencher.iter(|| solve(s, k));
}
