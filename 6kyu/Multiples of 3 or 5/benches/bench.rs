#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let num = black_box(45643);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::solution(num));
        }
    })
}
