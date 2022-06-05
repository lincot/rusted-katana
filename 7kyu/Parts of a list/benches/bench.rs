#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&["I", "wish", "I", "hadn't", "come"]);
    bencher.iter(|| solution::part_list(arr.to_vec()));
}
