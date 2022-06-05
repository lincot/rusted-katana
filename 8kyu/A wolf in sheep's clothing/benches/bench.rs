#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let queue = black_box(&["sheep", "sheep", "sheep", "wolf", "sheep", "sheep", "sheep"]);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::warn_the_sheep(queue));
        }
    });
}
