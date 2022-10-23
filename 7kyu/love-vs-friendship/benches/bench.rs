#![no_std]
#![feature(test)]

extern crate test;
use love_vs_friendship::words_to_marks;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(words_to_marks(black_box("attitude")));
        }
    });
}
