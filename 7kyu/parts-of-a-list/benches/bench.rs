#![feature(test)]

extern crate test;
use parts_of_a_list::part_list;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&["I", "wish", "I", "hadn't", "come"]);
    bencher.iter(|| part_list(arr.to_vec()));
}
