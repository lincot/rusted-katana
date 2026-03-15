#![feature(test)]

extern crate test;
use even_or_odd::even_or_odd;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| even_or_odd(black_box(5)));
}
