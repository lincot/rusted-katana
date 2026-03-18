#![feature(test)]

extern crate test;
use resistor_color_codes::decode_resistor_colors;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| decode_resistor_colors(black_box("yellow violet orange gold")));
}
