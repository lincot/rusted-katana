#![feature(test)]

extern crate test;
use reverse_a_number_in_any_base::reversed_number;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| reversed_number(black_box(3_040_620_649), black_box(7)));
}
