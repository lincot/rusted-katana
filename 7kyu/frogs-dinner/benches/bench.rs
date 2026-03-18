#![feature(test)]

extern crate test;
use frogs_dinner::frog_contest;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| frog_contest(black_box(32)));
}
