#![no_std]
#![feature(test)]

extern crate test;
use hello_name_or_world::hello;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("алиСА");
    bencher.iter(|| hello(s));
}
