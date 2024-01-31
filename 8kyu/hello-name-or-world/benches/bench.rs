#![feature(test)]

extern crate test;
use hello_name_or_world::hello;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| hello(black_box("алиСА")));
}
