#![no_std]
#![feature(test)]

extern crate test;
use count_ones_in_a_segment::count_ones;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let left = black_box(88_072_175_798_639);
    let right = black_box(112_156_148_935_024);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(count_ones(left, right));
        }
    });
}
