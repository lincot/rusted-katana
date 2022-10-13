#![no_std]
#![feature(test)]

extern crate test;
use multiples_by_permutations_ii::find_lowest_int;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for k in 1..10_000 {
            black_box(find_lowest_int(k));
        }
    });
}
