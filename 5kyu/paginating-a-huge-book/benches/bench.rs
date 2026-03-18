#![feature(test)]

extern crate test;
use paginating_a_huge_book::page_digits;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| page_digits(black_box(400_000_000_000_000_000)));
}
