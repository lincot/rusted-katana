#![no_std]
#![feature(test)]

extern crate test;
use scaling_squared_strings::scale;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        scale(
            black_box("abcd\nefghefgh\nijkl\nSH\nqrst"),
            black_box(10),
            black_box(10),
        )
    });
}
