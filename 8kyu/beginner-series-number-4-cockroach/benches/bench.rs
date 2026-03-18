#![feature(test)]

extern crate test;
use beginner_series_number_4_cockroach::cockroach_speed;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| cockroach_speed(black_box(10.)));
}
