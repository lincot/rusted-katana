#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use game_hit_the_target::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        solution(black_box(&[
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', ' ', ' ', ' '],
            vec![' ', ' ', '>', ' ', 'x'],
            vec![' ', ' ', ' ', ' ', ' '],
        ]))
    });
}
