#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use typing_series_number_1_the_backspace_function::solve;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        solve(black_box(
            "oop[backspace]*1oo[backspace]*2pppp[backspace]*3s",
        ))
    });
}
