#![feature(test)]

extern crate test;
use how_many_pages_in_a_book::amount_of_pages;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| amount_of_pages(black_box(488_895)));
}
