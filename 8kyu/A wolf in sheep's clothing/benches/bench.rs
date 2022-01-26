#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const QUEUE: [&str; 7] = ["sheep", "sheep", "sheep", "wolf", "sheep", "sheep", "sheep"];

#[bench]
fn bench(b: &mut Bencher) {
    let queue = black_box(&QUEUE);

    b.iter(|| {
        for _ in 0..1000 {
            black_box(solution::warn_the_sheep(queue));
        }
    })
}
