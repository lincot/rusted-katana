#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const WIDTH: u64 = 1024;
const RATIO: &str = "4:3";

#[bench]
fn bench(bencher: &mut Bencher) {
    let width = black_box(WIDTH);
    let ratio = black_box(RATIO);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::find_screen_height(width, ratio));
        }
    })
}
