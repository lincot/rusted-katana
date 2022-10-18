#![no_std]
#![feature(test)]

extern crate test;
use easy_wallpaper::wall_paper;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| wall_paper(black_box(6.3), black_box(4.5), black_box(3.29)));
}
