#![feature(test)]

extern crate test;
use are_you_playing_banjo::are_you_playing_banjo;
use test::{black_box, Bencher};

#[bench]
fn bench_martin(bencher: &mut Bencher) {
    bencher.iter(|| are_you_playing_banjo(black_box("Martin")));
}

#[bench]
fn bench_rikke(bencher: &mut Bencher) {
    bencher.iter(|| are_you_playing_banjo(black_box("Rikke")));
}
