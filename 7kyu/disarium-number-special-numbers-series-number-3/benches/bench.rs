#![feature(test)]

extern crate test;
use disarium_number_special_numbers_series_number_3::disarium_number;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| disarium_number(black_box(136_586)));
}
