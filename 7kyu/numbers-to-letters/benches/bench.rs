#![feature(test)]

extern crate test;
use numbers_to_letters::switcher;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let numbers = black_box(&[
        "25", "7", "8", "4", "14", "23", "8", "25", "23", "29", "16", "16", "4",
    ]);
    bencher.iter(|| switcher(numbers.to_vec()));
}
