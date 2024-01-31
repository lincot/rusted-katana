#![feature(test)]

extern crate test;
use apparently_modifying_strings::apparently;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| apparently(black_box("It was great and I have never been on live television before but sometimes I dont watch this.")));
}
