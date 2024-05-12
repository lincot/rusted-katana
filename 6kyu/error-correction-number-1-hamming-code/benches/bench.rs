#![feature(test)]

extern crate test;
use error_correction_number_1_hamming_code::{decode, encode};
use test::{black_box, Bencher};

#[bench]
fn bench_encode(bencher: &mut Bencher) {
    bencher.iter(|| encode(black_box("The Sensei told me that i can do this kata")));
}

#[bench]
fn bench_decode(bencher: &mut Bencher) {
    bencher.iter(|| decode(black_box("000111000111000111000100000111111000111000001000000111111000010111000111000100111000000000000100000111000111000000111111000111111000000111000111000111111000111111111000000111111111000000111111000110111000000111000111000111111000111000000111000000111000000000000000000111111111000111000000000111111000111111111111000111111000111111000000000111111000000111000001000000111000000000001000000111111000111111000111000111111000000111000111000000111000000000000000000111111111000111000000000111111000111000000000000111111000000010000111000111111111000111000000000100111000000000000000000111111000111000000111000000111000000000000000000111111000000000111111000111111000000000000111000111111000111111111000000000111000000000000010000111111000000111000000000111111000111111110111000000111000000000000000000111111111000111000000000111111000111000000000000111111000111000000111000111111111000000111111000000111000000000000000000111111000111000111111000111111000000000000111000111111111000111000000000111111000000000000111")));
}
