#![feature(test)]

extern crate test;
use multiples_by_permutations::search_perm_mult;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for k in 3..=7 {
            black_box(search_perm_mult(
                black_box(if cfg!(miri) { 100 } else { 10_000 }),
                black_box(k),
            ));
        }
    });
}
