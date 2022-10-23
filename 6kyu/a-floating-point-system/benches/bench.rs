#![no_std]
#![feature(test)]

extern crate test;
use a_floating_point_system::mant_exp;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        mant_exp(black_box("293873587705571899083980308388340775236183060925091306764344925255911665297217970136678400.0"), black_box(15));
        mant_exp(black_box("0.4567001"), black_box(4));
        mant_exp(black_box("0.0006"), black_box(15));
        mant_exp(black_box("1.000061234598911"), black_box(10));
        mant_exp(black_box("1.0000612345989"), black_box(100));
    });
}
