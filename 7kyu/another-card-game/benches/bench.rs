#![feature(test)]

extern crate test;
use another_card_game::the_game;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        the_game(
            black_box(&[1, 2, 3, 4]),
            black_box(&[5, 6, 7, 8]),
            black_box(&[0, 9, 10, 11]),
        )
    });
}
