#![feature(test)]

extern crate test;
use pirates_are_the_cannons_ready::cannons_ready;
use std::collections::HashMap;
use test::{black_box, Bencher};

#[bench]
fn bench_ready(bencher: &mut Bencher) {
    let gunners = black_box(HashMap::from([
        ("Mike", "aye"),
        ("Joe", "aye"),
        ("Johnson", "aye"),
        ("Peter", "aye"),
    ]));
    bencher.iter(|| cannons_ready(gunners.clone()));
}

#[bench]
fn bench_not_ready(bencher: &mut Bencher) {
    let gunners = black_box(HashMap::from([
        ("Mike", "aye"),
        ("Joe", "nay"),
        ("Johnson", "aye"),
        ("Peter", "aye"),
    ]));
    bencher.iter(|| cannons_ready(gunners.clone()));
}
