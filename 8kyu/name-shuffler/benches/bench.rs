#![no_std]
#![feature(test)]

extern crate test;
use name_shuffler::name_shuffler;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| name_shuffler(black_box("Дмитрий Муратов")));
}
