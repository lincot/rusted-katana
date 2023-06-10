#![no_std]
#![feature(test)]

extern crate test;
use remove_first_and_last_character::remove_char;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(remove_char(black_box("Путин")));
        }
    });
}
