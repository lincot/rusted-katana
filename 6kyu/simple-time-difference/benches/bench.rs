#![feature(test)]

extern crate test;
use simple_time_difference::solve;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| solve(black_box(&["21:14", "15:34", "14:51", "06:25", "15:30"])));
}
