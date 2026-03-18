#![feature(test)]

extern crate test;
use alphabetically_ordered::alphabetic;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| alphabetic(black_box("abcdefghijklmnop")));
}
