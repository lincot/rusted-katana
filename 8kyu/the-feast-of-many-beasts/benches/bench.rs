#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_feast_of_many_beasts::feast;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| feast(black_box("медвед"), black_box("мёд")));
}
