#![feature(test)]

extern crate test;
use ball_upwards::max_ball;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(max_ball(black_box(283)));
        }
    });
}
