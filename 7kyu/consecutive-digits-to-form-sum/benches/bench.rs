#![feature(test)]

extern crate test;
use consecutive_digits_to_form_sum::consecutive_ducks;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| consecutive_ducks(black_box(1006)));
}
