#![feature(test)]

extern crate test;
use sentence_smash::smash;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| smash(black_box(&["hello", "world", "this", "is", "great"])));
}
