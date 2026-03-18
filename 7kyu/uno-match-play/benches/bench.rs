#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use uno_match_play::can_play;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        can_play(
            black_box(&["yellow 3", "blue 5", "red 8", "red 9"]),
            black_box("green 4"),
        )
    });
}
