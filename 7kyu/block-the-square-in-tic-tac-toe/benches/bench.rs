#![no_std]
#![feature(test)]

extern crate test;
use block_the_square_in_tic_tac_toe::block_player;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for (a, b, c) in [
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (0, 4, 8),
            (2, 4, 6),
        ] {
            black_box(block_player(black_box(a), black_box(b)));
            black_box(block_player(black_box(b), black_box(c)));
            black_box(block_player(black_box(a), black_box(c)));
            black_box(block_player(black_box(b), black_box(a)));
            black_box(block_player(black_box(c), black_box(b)));
            black_box(block_player(black_box(c), black_box(a)));
        }
    });
}
