#![no_std]
#![feature(test)]

extern crate test;
use minimum_perimeter_of_a_rectangle::minimum_perimeter;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let area = black_box(27720);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(minimum_perimeter(area));
        }
    });
}
