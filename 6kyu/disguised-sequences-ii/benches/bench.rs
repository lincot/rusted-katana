#![feature(test)]

extern crate test;
use disguised_sequences_ii::{u1, v1};
use test::{Bencher, black_box};

#[bench]
fn bench_v1(bencher: &mut Bencher) {
    bencher.iter(|| v1(black_box(5), black_box(65)));
}

#[bench]
fn bench_u1(bencher: &mut Bencher) {
    bencher.iter(|| u1(black_box(5), black_box(65)));
}
