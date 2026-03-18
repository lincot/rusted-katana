#![feature(test)]

extern crate test;
use simple_game::game;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| game(black_box(3), black_box(7)));
}
