#![feature(test)]

extern crate test;
use allergy_to_palindromes::count_pal;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| black_box(count_pal(black_box(70))));
}
