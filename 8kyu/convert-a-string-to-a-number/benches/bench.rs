#![feature(test)]

extern crate test;
use convert_a_string_to_a_number::string_to_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| string_to_number(black_box("12345678")));
}
