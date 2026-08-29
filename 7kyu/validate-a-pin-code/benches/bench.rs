#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use validate_a_pin_code::validate_pin;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(validate_pin(black_box("123456")));
        black_box(validate_pin(black_box("1111")));
        black_box(validate_pin(black_box("a234")));
    });
}
