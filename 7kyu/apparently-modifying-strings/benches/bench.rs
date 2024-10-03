#![feature(test)]

extern crate test;
use apparently_modifying_strings::apparently;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const STRING: &str = "It was great and I have never been on live television before but sometimes I dont watch this.";

    bencher.iter(|| apparently(black_box(STRING)));
}
