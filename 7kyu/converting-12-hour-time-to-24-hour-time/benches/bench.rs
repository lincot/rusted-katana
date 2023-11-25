#![no_std]
#![feature(test)]

extern crate test;
use converting_12_hour_time_to_24_hour_time::to24hourtime;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(to24hourtime(black_box(2), black_box(15), black_box("am")));
            black_box(to24hourtime(black_box(12), black_box(31), black_box("am")));
            black_box(to24hourtime(black_box(7), black_box(50), black_box("pm")));
            black_box(to24hourtime(black_box(12), black_box(20), black_box("pm")));
        }
    });
}
