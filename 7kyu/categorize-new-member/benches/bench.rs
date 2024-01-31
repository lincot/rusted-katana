#![feature(test)]

extern crate test;
use categorize_new_member::open_or_senior;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| open_or_senior(black_box(vec![(45, 12), (55, 21), (19, -2), (104, 20)])));
}
