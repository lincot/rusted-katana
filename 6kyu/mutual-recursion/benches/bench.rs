#![feature(test)]

extern crate test;
use mutual_recursion::{f, m};
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(black_box(f(10)));
        black_box(black_box(m(10)));
    });
}
