#![no_std]
#![feature(test)]

extern crate test;
use divide_and_conquer::div_con;
use either::Either;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(div_con(black_box(&[
                Either::Right("3".into()),
                Either::Left(6),
                Either::Left(6),
                Either::Left(0),
                Either::Right("5".into()),
                Either::Left(8),
                Either::Left(5),
                Either::Right("6".into()),
                Either::Left(2),
                Either::Right("0".into()),
            ])));
        }
    });
}
