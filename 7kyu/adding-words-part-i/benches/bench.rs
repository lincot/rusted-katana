#![no_std]
#![feature(test)]

extern crate test;
use adding_words_part_i::Arith;
use core::ops::Add;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(black_box(Arith { value: "eight" }).add(black_box("five")));
        }
    });
}
