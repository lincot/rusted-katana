#![feature(test)]

extern crate test;
use basic_mathematical_operations::basic_op;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| basic_op(black_box('-'), black_box(256), black_box(32)));
}
