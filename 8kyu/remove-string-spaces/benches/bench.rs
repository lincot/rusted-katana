#![no_std]
#![feature(test)]

extern crate test;
use remove_string_spaces::no_space;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| no_space(black_box("8 j 8   mBliB8g  imjB8B8  jl  B".into())));
}
