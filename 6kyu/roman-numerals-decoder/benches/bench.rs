#![feature(test)]

extern crate test;
use roman_numerals_decoder::roman_as_num;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(roman_as_num(black_box("MMVIII")));
        black_box(roman_as_num(black_box("CDXLIV")));
    });
}
