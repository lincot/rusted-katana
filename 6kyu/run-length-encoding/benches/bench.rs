#![no_std]
#![feature(test)]

extern crate test;
use run_length_encoding::run_length_encoding;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| run_length_encoding(black_box("ааввыыыыввфжжжррыбнннфхххандустлвыхмйй")));
}
