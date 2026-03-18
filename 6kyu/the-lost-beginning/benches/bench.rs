#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use the_lost_beginning::find;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find(black_box("99100101102103")));
}
