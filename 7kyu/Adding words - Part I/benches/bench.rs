#![feature(test)]

extern crate test;
use std::ops::Add;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let c = black_box(solution::Arith { value: "eight" });
    let s = black_box("five");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(c.clone().add(s));
        }
    })
}
