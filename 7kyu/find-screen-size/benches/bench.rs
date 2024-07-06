#![feature(test)]

extern crate test;
use find_screen_size::find_screen_height;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_screen_height(black_box(1024), black_box("4:3")));
}
