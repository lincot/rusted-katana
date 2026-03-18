#![feature(test)]

extern crate test;
use plural::plural;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| plural(black_box(123_123_123_123_123_123.)));
}
