#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use trimming_a_string::trim;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| trim(black_box("Создавать каты прикольно"), black_box(14)));
}
