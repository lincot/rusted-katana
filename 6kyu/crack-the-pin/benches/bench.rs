#![no_std]
#![feature(test)]

extern crate test;
use crack_the_pin::crack;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| crack(black_box("c1ec8dd44a4f9c19fe8c7ae9b5592d24".into())));
}
