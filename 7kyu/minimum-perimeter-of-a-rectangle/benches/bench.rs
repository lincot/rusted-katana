#![feature(test)]

extern crate test;
use minimum_perimeter_of_a_rectangle::minimum_perimeter;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| minimum_perimeter(black_box(27720)));
}
