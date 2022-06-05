#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box(12);
    let x = black_box(&[
        0.0, 0.11, 0.22, 0.33, 0.44, 0.65, 1.08, 1.26, 1.68, 1.89, 2.1, 2.31, 2.52, 3.25,
    ]);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::gps(s, x.to_vec()));
        }
    });
}
