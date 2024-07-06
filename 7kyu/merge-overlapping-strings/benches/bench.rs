#![feature(test)]

extern crate test;
use merge_overlapping_strings::merge_strings;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| merge_strings(black_box("https://example.com/a"), black_box("/a/test")));
}
