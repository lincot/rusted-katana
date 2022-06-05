#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(136);
    let b = black_box(35);
    let c = black_box(116);
    let d = black_box(375);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::prod2sum(a, b, c, d));
        }
    });
}
