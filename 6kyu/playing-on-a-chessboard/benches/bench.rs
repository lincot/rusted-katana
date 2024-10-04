#![feature(test)]

extern crate test;
use playing_on_a_chessboard::game;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| game(black_box(807)));
}
