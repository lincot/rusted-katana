#![feature(test)]

extern crate test;
use roman_numerals_encoder::num_as_roman;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for num in 1..=if cfg!(miri) { 15 } else { 3999 } {
            black_box(num_as_roman(black_box(num)));
        }
    });
}
