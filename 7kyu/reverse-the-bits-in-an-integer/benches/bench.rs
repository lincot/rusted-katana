#![feature(test)]

extern crate test;
use reverse_the_bits_in_an_integer::reverse_bits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| reverse_bits(black_box(1023)));
}
