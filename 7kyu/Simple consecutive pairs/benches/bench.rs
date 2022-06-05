#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&[21, 20, 22, 40, 39, -56, 30, -55, 95, 94]);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::pairs(arr));
        }
    });
}
