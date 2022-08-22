#![feature(test)]

extern crate test;
use string_doubles::doubles;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let s = black_box("ххбннннныааааам");
    bencher.iter(|| doubles(s));
}