#![no_std]
#![feature(test)]

extern crate test;
use roboscript_number_1_implement_syntax_highlighting::highlight;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| highlight(black_box("FFFR345F2LL")));
}
