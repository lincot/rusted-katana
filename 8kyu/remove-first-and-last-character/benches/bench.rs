#![feature(test)]

extern crate test;
use remove_first_and_last_character::remove_char;
use test::{Bencher, black_box};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| remove_char(black_box("Putin")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| remove_char(black_box("Путин")));
}
