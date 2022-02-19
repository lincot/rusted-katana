#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(8796203);
    let b = black_box(7556);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::solve(a, b));
        }
    })
}
