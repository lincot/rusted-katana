#![feature(test)]

extern crate test;
use string_repeat::repeat_str;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| repeat_str(black_box("string"), black_box(40)));
}
