#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_lost_beginning::find;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find(black_box("99100101102103")));
}
