#![no_std]
#![feature(test)]

extern crate test;
use rock_paper_scissors::rps;
use test::{black_box, Bencher};

#[bench]
fn bench1(bencher: &mut Bencher) {
    let p1 = black_box("rock");
    let p2 = black_box("paper");
    let p3 = black_box("scissors");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(rps(p1, p1));
            black_box(rps(p1, p2));
            black_box(rps(p1, p3));
            black_box(rps(p2, p1));
            black_box(rps(p2, p2));
            black_box(rps(p2, p3));
            black_box(rps(p3, p1));
            black_box(rps(p3, p2));
            black_box(rps(p3, p3));
        }
    });
}
