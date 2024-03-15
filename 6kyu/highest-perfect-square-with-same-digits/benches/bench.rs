#![feature(test)]

extern crate test;
use highest_perfect_square_with_same_digits::next_perfectsq_perm;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        next_perfectsq_perm(
            black_box(if cfg!(miri) { 100 } else { 100_000 }),
            black_box(if cfg!(miri) { 2 } else { 4 }),
        )
    });
}
