#![feature(test)]

extern crate test;
use leap_years::is_leap_year;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(is_leap_year(black_box(2000)));
        black_box(is_leap_year(black_box(2015)));
    });
}
