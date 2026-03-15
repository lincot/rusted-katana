#![feature(test)]

extern crate test;
use compare_within_margin::close_compare;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for (a, b, margin) in [(1., 2., 3.), (1., 2., 0.5), (20., 5., 2.)] {
            black_box(close_compare(black_box(a), black_box(b), black_box(margin)));
        }
    });
}
