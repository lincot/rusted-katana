#![feature(test)]

extern crate test;
use detect_pangram::is_pangram;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| is_pangram(black_box("How quickly daft jumping zebras vex.")));
}
