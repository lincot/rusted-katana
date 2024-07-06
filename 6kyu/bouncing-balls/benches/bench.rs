#![feature(test)]

extern crate test;
use bouncing_balls::bouncing_ball;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(bouncing_ball(
            black_box(30.),
            black_box(0.66),
            black_box(1.5),
        ));
    });
}
