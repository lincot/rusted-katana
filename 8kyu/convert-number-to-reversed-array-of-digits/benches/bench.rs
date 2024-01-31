#![feature(test)]

extern crate test;
use convert_number_to_reversed_array_of_digits::digitize;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(digitize(black_box(456_486_734_798)));
        }
    });
}
