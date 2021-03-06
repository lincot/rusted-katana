#![feature(test)]

extern crate test;
use multiples_of_3_or_5::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let num = black_box(45643);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution(num));
        }
    });
}
