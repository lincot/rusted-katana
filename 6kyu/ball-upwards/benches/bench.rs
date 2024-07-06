#![feature(test)]

extern crate test;
use ball_upwards::max_ball;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| max_ball(black_box(283)));
}
