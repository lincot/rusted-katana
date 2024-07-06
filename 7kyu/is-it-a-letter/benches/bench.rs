#![feature(test)]

extern crate test;
use is_it_a_letter::is_it_letter;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| is_it_letter(black_box('a')));
}
