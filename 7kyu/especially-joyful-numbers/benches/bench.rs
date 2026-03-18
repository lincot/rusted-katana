#![feature(test)]

extern crate test;
use especially_joyful_numbers::number_joy;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| number_joy(black_box(1997)));
}
