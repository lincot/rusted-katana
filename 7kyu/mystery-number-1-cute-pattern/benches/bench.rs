#![feature(test)]

extern crate test;
use mystery_number_1_cute_pattern::cute_pattern;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| cute_pattern(black_box("BWWB\nWBWW\nWWBW\nBWWB")));
}
