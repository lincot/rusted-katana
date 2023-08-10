#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_feast_of_many_beasts::feast;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(feast(black_box("медвед"), black_box("мёд")));
        }
    });
}
