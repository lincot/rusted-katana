#![feature(test)]

extern crate test;
use count_ones_in_a_segment::count_ones;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_ones(
            black_box(88_072_175_798_639),
            black_box(112_156_148_935_024),
        )
    });
}
