#![no_std]
#![feature(test)]

extern crate test;
use adding_words_part_i::Arith;
use core::ops::Add;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let c = black_box(Arith { value: "eight" });
    let s = black_box("five");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(c.clone().add(s));
        }
    });
}
