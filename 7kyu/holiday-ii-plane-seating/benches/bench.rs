#![no_std]
#![feature(test)]

extern crate test;
use holiday_ii_plane_seating::plane_seat;
use test::{black_box, Bencher};

#[bench]
fn bench_valid(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(plane_seat(black_box("20B")));
        }
    });
}

#[bench]
fn bench_invalid(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(plane_seat(black_box("58I")));
        }
    });
}
