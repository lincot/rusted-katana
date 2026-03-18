#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use welcome::greet;

#[bench]
fn bench_russian(bencher: &mut Bencher) {
    bencher.iter(|| greet(black_box("russian")));
}

#[bench]
fn bench_swedish(bencher: &mut Bencher) {
    bencher.iter(|| greet(black_box("swedish")));
}
