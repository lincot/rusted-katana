#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_spiraling_box::create_box;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| create_box(black_box(100), black_box(100)));
}
