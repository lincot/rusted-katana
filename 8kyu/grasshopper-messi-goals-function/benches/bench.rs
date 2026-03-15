#![feature(test)]

extern crate test;
use grasshopper_messi_goals_function::goals;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| goals(black_box(30), black_box(40), black_box(50)));
}
