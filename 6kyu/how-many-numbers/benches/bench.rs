#![feature(test)]

extern crate test;
use how_many_numbers::sel_number;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| sel_number(black_box(123_456), black_box(3)));
}
