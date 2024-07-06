#![feature(test)]

extern crate test;
use convert_a_boolean_to_a_string::boolean_to_string;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| boolean_to_string(black_box(true)));
}
