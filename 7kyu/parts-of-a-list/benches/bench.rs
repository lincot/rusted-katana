#![feature(test)]

extern crate test;
use parts_of_a_list::part_list;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| part_list(black_box(vec!["I", "wish", "I", "hadn't", "come"])));
}
