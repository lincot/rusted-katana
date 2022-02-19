#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let month = black_box(3);
    bencher.iter(|| {
        for _ in 0..1_000_000 {
            black_box(solution::quarter_of(month));
        }
    })
}
