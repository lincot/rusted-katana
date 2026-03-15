#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use will_there_be_enough_space::enough;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| enough(black_box(100), black_box(60), black_box(50)));
}
