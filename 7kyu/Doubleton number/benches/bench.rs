#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for i in 1..=1_000_000 {
            black_box(solution::doubleton(i));
        }
    })
}
