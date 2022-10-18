#![no_std]
#![feature(test)]

extern crate test;
use hashbrown::HashMap;
use pirates_are_the_cannons_ready::cannons_ready;
use test::{black_box, Bencher};

#[bench]
fn bench_ready(bencher: &mut Bencher) {
    bencher.iter(|| {
        cannons_ready(black_box(HashMap::from([
            ("Mike", "aye"),
            ("Joe", "aye"),
            ("Johnson", "aye"),
            ("Peter", "aye"),
        ])))
    });
}

#[bench]
fn bench_not_ready(bencher: &mut Bencher) {
    bencher.iter(|| {
        cannons_ready(black_box(HashMap::from([
            ("Mike", "aye"),
            ("Joe", "nay"),
            ("Johnson", "aye"),
            ("Peter", "aye"),
        ])))
    });
}
