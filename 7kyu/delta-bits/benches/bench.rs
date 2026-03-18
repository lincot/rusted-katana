#![feature(test)]

extern crate test;
use delta_bits::convert_bits;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(convert_bits(black_box(31), black_box(14)));
        black_box(convert_bits(black_box(7), black_box(17)));
    });
}
