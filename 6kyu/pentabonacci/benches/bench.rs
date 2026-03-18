#![feature(test)]

extern crate test;
use pentabonacci::count_odd_pentafib;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| count_odd_pentafib(black_box(45)));
}
