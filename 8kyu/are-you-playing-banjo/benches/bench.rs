#![no_std]
#![feature(test)]

extern crate test;
use are_you_playing_banjo::are_you_playing_banjo;
use test::{black_box, Bencher};

#[bench]
fn bench_martin(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(are_you_playing_banjo(black_box("Martin")));
        }
    });
}

#[bench]
fn bench_rikke(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(are_you_playing_banjo(black_box("Rikke")));
        }
    });
}
