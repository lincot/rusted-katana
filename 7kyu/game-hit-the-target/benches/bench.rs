#![feature(test)]

extern crate test;
use game_hit_the_target::solution;
use test::{Bencher, black_box};

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
