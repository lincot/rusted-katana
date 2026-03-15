#![feature(test)]

extern crate test;
use simple_game::game;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| game(black_box(3), black_box(7)));
}
