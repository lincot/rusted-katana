#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const MARTIN: &str = "Martin";
const RIKKE: &str = "Rikke";

#[bench]
fn martin_bench(b: &mut Bencher) {
    let name = black_box(MARTIN);

    b.iter(|| black_box(solution::are_you_playing_banjo(name)))
}

#[bench]
fn rikke_bench(b: &mut Bencher) {
    let name = black_box(RIKKE);

    b.iter(|| black_box(solution::are_you_playing_banjo(name)))
}
