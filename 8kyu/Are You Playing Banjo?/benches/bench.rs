#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const MARTIN: &str = "Martin";
const RIKKE: &str = "Rikke";

#[bench]
fn martin_bench(bencher: &mut Bencher) {
    let name = black_box(MARTIN);

    bencher.iter(|| solution::are_you_playing_banjo(name))
}

#[bench]
fn rikke_bench(bencher: &mut Bencher) {
    let name = black_box(RIKKE);

    bencher.iter(|| solution::are_you_playing_banjo(name))
}
