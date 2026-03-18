#![feature(test)]

extern crate test;
use jennys_secret_message::greet;
use test::{Bencher, black_box};

#[bench]
fn bench_johny(bencher: &mut Bencher) {
    bencher.iter(|| greet(black_box("Johnny")));
}

#[bench]
fn bench_susan(bencher: &mut Bencher) {
    bencher.iter(|| greet(black_box("Susan")));
}
