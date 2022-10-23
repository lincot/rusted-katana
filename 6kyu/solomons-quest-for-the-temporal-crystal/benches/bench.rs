#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use solomons_quest_for_the_temporal_crystal::quest::solomons_quest;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        solomons_quest(black_box(vec![
            (3, 2, 80),
            (1, 1, 25),
            (6, 0, 8),
            (-5, 3, 50),
            (1, 2, 100),
            (4, 0, 9),
            (-8, 3, 260),
            (0, 1, 90),
        ]))
    });
}
