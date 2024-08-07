#![feature(test)]

extern crate test;
use prime_reduction::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        solve(
            black_box(100),
            black_box(if cfg!(miri) { 200 } else { 50_000 }),
        )
    });
}
