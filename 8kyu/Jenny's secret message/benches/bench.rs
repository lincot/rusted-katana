#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const JOHNNY: &str = "Johnny";
const SUSAN: &str = "Susan";

#[bench]
fn johnny_bench(bencher: &mut Bencher) {
    let input = black_box(JOHNNY);

    bencher.iter(|| solution::greet(input))
}

#[bench]
fn susan_bench(bencher: &mut Bencher) {
    let input = black_box(SUSAN);

    bencher.iter(|| solution::greet(input))
}
