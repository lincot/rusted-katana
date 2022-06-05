#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_decode(bencher: &mut Bencher) {
    let s = black_box("01110111011111000110010011110011110011110011110011110011110111011101110110011001100110011001101111111010101100011001000110000001100000011000");
    bencher.iter(|| solution::decode(s));
}

#[bench]
fn bench_code(bencher: &mut Bencher) {
    let s = black_box("3331977777733322222211100019888");
    bencher.iter(|| solution::code(s));
}
