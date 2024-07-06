#![feature(test)]

extern crate test;
use adding_words_part_i::Arith;
use core::ops::Add;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Arith { value: "eight" }).add(black_box("five")));
}
