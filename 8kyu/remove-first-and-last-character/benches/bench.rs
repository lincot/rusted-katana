#![no_std]
#![feature(test)]

extern crate test;
use remove_first_and_last_character::remove_char;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("Путин");
    bencher.iter(|| remove_char(s));
}
