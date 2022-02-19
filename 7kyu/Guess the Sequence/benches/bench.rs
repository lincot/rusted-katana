#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for x in 1..=100 {
            black_box(solution::sequence(x));
        }
    })
}
