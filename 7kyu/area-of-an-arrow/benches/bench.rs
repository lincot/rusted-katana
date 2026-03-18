#![feature(test)]

extern crate test;
use area_of_an_arrow::arrow_area;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| arrow_area(black_box(25), black_box(25)));
}
