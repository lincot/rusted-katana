#![feature(test)]

extern crate test;
use expressions_matter::expressions_matter;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(expressions_matter(black_box(5), black_box(6), black_box(7)));
        black_box(expressions_matter(black_box(1), black_box(6), black_box(1)));
        black_box(expressions_matter(black_box(1), black_box(6), black_box(5)));
        black_box(expressions_matter(black_box(5), black_box(6), black_box(1)));
    });
}
