#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use word_to_initial_number::convert;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| convert(black_box("КрАсАвЧиК")));
}
