#![no_std]
#![feature(test)]

extern crate test;
use find_screen_size::find_screen_height;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let width = black_box(1024);
    let ratio = black_box("4:3");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(find_screen_height(width, ratio));
        }
    });
}
