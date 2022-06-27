#![feature(test)]

extern crate test;
use barista_problem::barista;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let coffees = black_box(&[
        87, 135, 147, 228, 130, 192, 205, 64, 149, 140, 71, 212, 202, 58, 125, 183, 173, 131, 123,
        26, 231, 80, 220, 27, 129, 114, 119, 61, 179, 70, 231, 139, 43, 43, 153, 147, 137, 213,
        164, 245, 86, 166, 203, 69, 225, 132, 96, 240, 187, 125, 219, 254, 23, 125, 27, 159, 95,
        85, 230, 33, 148, 117, 88, 220, 72, 196, 201, 124, 25, 11, 179, 179, 217, 219, 253, 135,
        18, 19, 192, 170, 100, 237, 10, 73, 163, 49, 30, 56, 32, 61, 225, 153, 95, 30, 129, 232,
        109, 50, 176, 175,
    ]);
    bencher.iter(|| barista(coffees));
}
