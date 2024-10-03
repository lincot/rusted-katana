#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_feast_of_many_beasts::feast;

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| feast(black_box("chickadee"), black_box("chocolate cake")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| feast(black_box("медвед"), black_box("мёд")));
}
