#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_martin(bencher: &mut Bencher) {
    let name = black_box("Martin");
    bencher.iter(|| solution::are_you_playing_banjo(name))
}

#[bench]
fn bench_rikke(bencher: &mut Bencher) {
    let name = black_box("Rikke");
    bencher.iter(|| solution::are_you_playing_banjo(name))
}
