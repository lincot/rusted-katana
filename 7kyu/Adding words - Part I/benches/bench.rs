#![feature(test)]

extern crate test;
use solution::*;
use std::ops::Add;
use test::{black_box, Bencher};

const C: Arith = Arith { value: "eight" };
const S: &str = "five";

#[bench]
fn bench(bencher: &mut Bencher) {
    let c = black_box(C);
    let s = black_box(S);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(c.clone().add(s));
        }
    })
}
