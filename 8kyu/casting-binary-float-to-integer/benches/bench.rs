#![feature(test)]

extern crate test;
use casting_binary_float_to_integer::convert_to_i32;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| convert_to_i32(black_box(-256.123)));
}
