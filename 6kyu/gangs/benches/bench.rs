#![no_std]
#![feature(test)]

extern crate test;
use gangs::gangs;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(gangs(
            black_box(&[
                69, 33, 41, 17, 30, 12, 1, 15, 79, 34, 13, 3, 56, 23, 44, 93, 61, 62, 47, 74, 90,
                65, 89, 80, 25, 97, 87, 77, 72, 51, 29, 8, 99, 16, 40, 42, 24, 78, 85, 35, 49, 20,
                6, 76, 53, 37, 45, 59, 96, 98, 31, 28, 48, 10, 82, 58, 19, 71, 36, 38, 18, 88, 11,
                67, 9,
            ]),
            black_box(89),
        ))
    });
}
