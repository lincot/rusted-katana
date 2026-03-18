#![feature(test)]

extern crate test;
use resistor_color_codes_part_2::encode_resistor_colors;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| encode_resistor_colors(black_box("330k ohms")));
}
