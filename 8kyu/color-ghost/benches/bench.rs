#![feature(test)]

extern crate test;
use color_ghost::Ghost;
use test::Bencher;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(Ghost::new);
}
