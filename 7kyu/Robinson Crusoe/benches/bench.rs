#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const N: i32 = 45;
const D: f64 = 0.1;
const ANG: i32 = 3;
const DISTMULT: f64 = 1.01;
const ANGMULT: f64 = 1.1;

#[bench]
fn bench(bencher: &mut Bencher) {
    let n = black_box(N);
    let d = black_box(D);
    let ang = black_box(ANG);
    let distmult = black_box(DISTMULT);
    let angmult = black_box(ANGMULT);

    bencher.iter(|| solution::crusoe(n, d, ang, distmult, angmult))
}
