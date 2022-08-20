#![feature(test)]

extern crate test;
use returning_strings::greet;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let name = black_box("Дмитрий");
    bencher.iter(|| greet(name));
}
