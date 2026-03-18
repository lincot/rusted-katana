#![feature(test)]

extern crate test;
use grasshopper_terminal_game_move_function::move_hero;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| move_hero(black_box(200), black_box(60)));
}
