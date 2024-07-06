#![feature(test)]

extern crate test;
use color_choice::check_choose;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| check_choose(black_box(184_756), black_box(20)));
}
