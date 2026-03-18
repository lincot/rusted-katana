#![feature(test)]

extern crate test;
use either::Either;
use sum_mixed_array::sum_mix;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = [
        Either::Right("5".into()),
        Either::Right("0".into()),
        Either::Left(9),
        Either::Left(3),
        Either::Left(2),
        Either::Left(1),
        Either::Right("9".into()),
        Either::Left(6),
        Either::Left(7),
    ];
    bencher.iter(|| sum_mix(black_box(&arr)));
}
