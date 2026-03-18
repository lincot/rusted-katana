#![feature(test)]

extern crate test;
use returning_strings::greet;
use test::{Bencher, black_box};

#[bench]
fn bench_ascii(bencher: &mut Bencher) {
    bencher.iter(|| greet(black_box("Dmitriy")));
}

#[bench]
fn bench_nonascii(bencher: &mut Bencher) {
    bencher.iter(|| greet(black_box("Дмитрий")));
}
