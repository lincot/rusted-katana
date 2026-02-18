#![feature(test)]

extern crate test;
use pyramid_array::pyramid;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| pyramid(black_box(if cfg!(miri) { 5 } else { 100 })));
}
