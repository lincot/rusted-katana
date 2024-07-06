#![feature(test)]

extern crate test;
use holiday_ii_plane_seating::plane_seat;
use test::{black_box, Bencher};

#[bench]
fn bench_valid(bencher: &mut Bencher) {
    bencher.iter(|| plane_seat(black_box("20B")));
}

#[bench]
fn bench_invalid(bencher: &mut Bencher) {
    bencher.iter(|| plane_seat(black_box("58I")));
}
