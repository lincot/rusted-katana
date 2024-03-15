#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_spider_and_the_fly_jumping_spider::spider_to_fly;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 0 } else { 100 } {
            black_box(spider_to_fly(black_box("H3"), black_box("E2")));
        }
    });
}
