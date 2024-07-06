#![feature(test)]

extern crate test;
use remove_first_and_last_character::remove_char;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| remove_char(black_box("Путин")));
}
