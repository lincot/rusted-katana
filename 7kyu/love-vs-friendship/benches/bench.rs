#![feature(test)]

extern crate test;
use love_vs_friendship::words_to_marks;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| words_to_marks(black_box("attitude")));
}
