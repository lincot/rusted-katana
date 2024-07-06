#![feature(test)]

extern crate test;
use find_the_missing_letter::find_missing_letter;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_missing_letter(black_box(&['a', 'b', 'c', 'd', 'f'])));
}
