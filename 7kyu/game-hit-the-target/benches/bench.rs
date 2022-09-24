#![no_std]
#![feature(test)]

extern crate alloc;
extern crate test;
use alloc::vec;
use game_hit_the_target::solution;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mtrx = [
        vec![' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', ' ', ' ', ' '],
        vec![' ', ' ', '>', ' ', 'x'],
        vec![' ', ' ', ' ', ' ', ' '],
    ];
    let mtrx = black_box(&mtrx);
    bencher.iter(|| solution(mtrx));
}
