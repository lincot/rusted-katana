#![no_std]
#![feature(test)]

extern crate test;
use rock_paper_scissors::rps;
use test::{black_box, Bencher};

#[bench]
fn bench1(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(rps(black_box("rock"), black_box("rock")));
            black_box(rps(black_box("rock"), black_box("paper")));
            black_box(rps(black_box("rock"), black_box("scissors")));
            black_box(rps(black_box("paper"), black_box("rock")));
            black_box(rps(black_box("paper"), black_box("paper")));
            black_box(rps(black_box("paper"), black_box("scissors")));
            black_box(rps(black_box("scissors"), black_box("rock")));
            black_box(rps(black_box("scissors"), black_box("paper")));
            black_box(rps(black_box("scissors"), black_box("scissors")));
        }
    });
}
