#![feature(test)]

extern crate test;
use hold_your_breath::diving_minigame;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(diving_minigame(black_box(&[
            -4, -3, -4, -3, 5, 2, -5, -20, -42, -4, 5, 3, 5,
        ])));
        black_box(diving_minigame(black_box(&[
            1, 2, 1, 2, 1, 2, 1, 2, 1, -3, -4, -5, -3, -4,
        ])));
        black_box(diving_minigame(black_box(&[
            20, 3, 4, -20, 14, 3, 8, -18, -20, -13, 13, -14, -12, -1, 20, -6, -20, -2,
        ])));
    });
}
