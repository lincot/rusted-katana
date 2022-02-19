#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const A: f64 = 134.;
const B: f64 = 191.5;
const C: f64 = 45.5;

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(A);
    let b = black_box(B);
    let c = black_box(C);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::shortest_distance(a, b, c));
        }
    })
}
