#![feature(test)]

extern crate test;
use std::collections::HashMap;
use test::{black_box, Bencher};

fn get_ready_gunners<'a>() -> HashMap<&'a str, &'a str> {
    HashMap::from([
        ("Mike", "aye"),
        ("Joe", "aye"),
        ("Johnson", "aye"),
        ("Peter", "aye"),
    ])
}

fn get_not_ready_gunners<'a>() -> HashMap<&'a str, &'a str> {
    HashMap::from([
        ("Mike", "aye"),
        ("Joe", "nay"),
        ("Johnson", "aye"),
        ("Peter", "aye"),
    ])
}

#[bench]
fn bench_ready(b: &mut Bencher) {
    let gunners = black_box(get_ready_gunners());

    b.iter(|| solution::cannons_ready(gunners.clone()))
}

#[bench]
fn bench_not_ready(b: &mut Bencher) {
    let gunners = black_box(get_not_ready_gunners());

    b.iter(|| solution::cannons_ready(gunners.clone()))
}
