#![feature(test)]

extern crate test;
use hurry_up_the_walls_are_closing_in::can_escape;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| can_escape(black_box(&[(2, 2), (3, 3), (4, 4), (2, 8), (7, 3), (6, 4)])));
}
