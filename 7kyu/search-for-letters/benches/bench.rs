#![feature(test)]

extern crate test;
use search_for_letters::change;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| change(black_box("function")));
}
