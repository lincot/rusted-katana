#![feature(test)]

extern crate test;
use finding_queens_on_the_board::queens;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| queens(black_box(3_123_214)));
}
