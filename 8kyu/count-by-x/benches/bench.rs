#![feature(test)]

extern crate test;
use count_by_x::count_by;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_by(
            black_box(5),
            black_box(if cfg!(miri) { 10 } else { 100_000 }),
        )
    });
}
