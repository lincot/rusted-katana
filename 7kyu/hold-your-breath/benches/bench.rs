#![feature(test)]

extern crate test;
use hold_your_breath::diving_minigame;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(diving_minigame(black_box(&[
            -4, -3, -4, -3, 5, 2, -5, -20, -42, -4, 5, 3, 5,
        ])));
        black_box(diving_minigame(black_box(&[
            1, 2, 1, 2, 1, 2, 1, 2, 1, -3, -4, -5, -3, -4,
        ])));
    });
}
