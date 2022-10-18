#![no_std]
#![feature(test)]

extern crate test;
use simple_letter_removal::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box("abracadabra"), black_box(6)));
}
