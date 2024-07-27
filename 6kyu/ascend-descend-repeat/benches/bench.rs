#![feature(test)]

extern crate test;
use ascend_descend_repeat::ascend_descend;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        ascend_descend(
            black_box(if cfg!(miri) { 100 } else { 10_000 }),
            black_box(if cfg!(miri) { -20 } else { -500 }),
            black_box(if cfg!(miri) { 20 } else { 500 }),
        )
    });
}
