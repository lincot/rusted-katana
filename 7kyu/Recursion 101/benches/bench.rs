#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const A: usize = 8796203;
const B: usize = 7556;

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(A);
    let b = black_box(B);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::solve(a, b));
        }
    })
}
