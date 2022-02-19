#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(134.);
    let b = black_box(191.5);
    let c = black_box(45.5);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::shortest_distance(a, b, c));
        }
    })
}
