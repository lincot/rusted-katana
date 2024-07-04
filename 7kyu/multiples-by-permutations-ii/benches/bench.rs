#![feature(test)]

extern crate test;
use multiples_by_permutations_ii::find_lowest_int;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for k in if cfg!(miri) { 2 } else { 1 }..if cfg!(miri) { 50 } else { 10_000 } {
            black_box(find_lowest_int(black_box(k)));
        }
    });
}
