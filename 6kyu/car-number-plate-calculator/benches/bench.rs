#![feature(test)]

extern crate test;
use car_number_plate_calculator::find_the_number_plate;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_the_number_plate(black_box(40_000)));
}
