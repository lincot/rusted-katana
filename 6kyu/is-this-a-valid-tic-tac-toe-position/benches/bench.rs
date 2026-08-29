#![feature(test)]

extern crate test;

use is_this_a_valid_tic_tac_toe_position::{Player::*, is_valid_position};
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| is_valid_position(black_box(&[[X, O, X], [X, O, X], [O, X, O]])));
}
