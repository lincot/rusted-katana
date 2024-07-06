#![feature(test)]

extern crate test;
use checkerboard_resolution::count_checkerboard;
use test::{black_box, Bencher};

#[bench]
fn bench_u64(bencher: &mut Bencher) {
    bencher.iter(|| count_checkerboard(black_box(11), black_box(6), black_box(5)));
}

#[bench]
fn bench_u128(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_checkerboard(
            black_box(8u128.pow(5)),
            black_box(7u128.pow(9)),
            black_box(124),
        )
    });
}
