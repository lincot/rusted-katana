#![feature(test)]

extern crate test;
use popping_blocks::pop_blocks;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let lst = [
        "b", "GV", "x", "x", "GV", "x", "GV", "qf", "qf", "GV", "ABP", "D", "shuE", "b", "GV", "D",
        "x", "b", "b", "shuE", "x", "GV", "qf", "ABP", "GV", "b", "b", "D", "D", "D", "shuE",
        "ABP", "ABP", "b", "b",
    ]
    .map(String::from);
    bencher.iter(|| pop_blocks(black_box(&lst)));
}
