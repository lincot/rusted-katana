#![no_std]
#![feature(test)]

extern crate test;
use easy_wallpaper::wall_paper;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let l = black_box(6.3);
    let w = black_box(4.5);
    let h = black_box(3.29);
    bencher.iter(|| wall_paper(l, w, h));
}
