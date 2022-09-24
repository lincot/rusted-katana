#![no_std]
#![feature(test)]

extern crate test;
use search_for_letters::change;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let string = black_box("function");
    bencher.iter(|| change(string));
}
