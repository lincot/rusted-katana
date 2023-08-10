#![no_std]
#![feature(test)]

extern crate test;
use find_screen_size::find_screen_height;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(find_screen_height(black_box(1024), black_box("4:3")));
        }
    });
}
