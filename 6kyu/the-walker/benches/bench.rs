#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use the_walker::solve;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        solve(
            black_box(15),
            black_box(15),
            black_box(19),
            black_box(50),
            black_box(29),
            black_box(55),
        )
    });
}
