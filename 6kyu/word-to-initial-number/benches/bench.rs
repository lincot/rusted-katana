#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use word_to_initial_number::convert;

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| convert(black_box("KrAsAvCiK")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| convert(black_box("КрАсАвЧиК")));
}
