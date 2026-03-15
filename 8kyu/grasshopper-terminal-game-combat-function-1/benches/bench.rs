#![feature(test)]

extern crate test;
use grasshopper_terminal_game_combat_function_1::combat;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| combat(black_box(100.), black_box(40.)));
}
