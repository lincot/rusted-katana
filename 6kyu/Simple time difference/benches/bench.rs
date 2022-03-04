#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&["21:14", "15:34", "14:51", "06:25", "15:30"]);
    bencher.iter(|| solution::solve(arr))
}
