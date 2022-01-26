#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const JOHNNY: &str = "Johnny";
const SUSAN: &str = "Susan";

#[bench]
fn johnny_bench(b: &mut Bencher) {
    let input = black_box(JOHNNY);

    b.iter(|| solution::greet(input))
}

#[bench]
fn susan_bench(b: &mut Bencher) {
    let input = black_box(SUSAN);

    b.iter(|| solution::greet(input))
}
