#![feature(test)]

extern crate test;
use mr_safetys_treasures::unlock;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| unlock(black_box("waterheater")));
}
