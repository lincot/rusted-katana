#![feature(test)]

extern crate test;
use convert_a_number_to_a_string::number_to_string;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| number_to_string(black_box(-1_893_660_523)));
}
