#![feature(test)]

extern crate test;
use robinson_crusoe::crusoe;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(45);
    let d = black_box(0.1);
    let ang = black_box(3);
    let distmult = black_box(1.01);
    let angmult = black_box(1.1);
    bencher.iter(|| crusoe(n, d, ang, distmult, angmult));
}
