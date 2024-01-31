#![feature(test)]

extern crate test;
use count_ones_in_a_segment::count_ones;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(count_ones(
                black_box(88_072_175_798_639),
                black_box(112_156_148_935_024),
            ));
        }
    });
}
