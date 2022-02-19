#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const A1: &str = "aretheyhere";
const A2: &str = "yestheyarehere";

#[bench]
fn bench(bencher: &mut Bencher) {
    let a1 = black_box(A1);
    let a2 = black_box(A2);

    bencher.iter(|| solution::longest(a1, a2))
}
