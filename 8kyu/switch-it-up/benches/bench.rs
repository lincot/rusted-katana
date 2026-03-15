#![feature(test)]

extern crate test;
use switch_it_up::switch_it_up;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| switch_it_up(black_box(3)));
}
