#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let xs = black_box(&[
        455, -75, -387, -101, 278, 143, -418, 64, -478, -323, -62, 302, -172, 470, -440, -340, 341,
        -218, 115, 353, 7, 38, 159, -281, -221, -421, -424, 483, 248, -219, -194, -23, -201, 77,
        -54, 110, -125, -79, 353, 461, -175, -283, -345, 3, 411, 131, 222, -320, 264, -67, -280,
        -349, -401,
    ]);
    bencher.iter(|| solution::pendulum(xs))
}
