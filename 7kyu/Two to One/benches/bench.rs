#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a1 = black_box("aretheyhere");
    let a2 = black_box("yestheyarehere");
    bencher.iter(|| solution::longest(a1, a2))
}
