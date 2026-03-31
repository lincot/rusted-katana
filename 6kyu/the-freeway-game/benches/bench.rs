#![feature(test)]

extern crate test;
use core::array;

use test::{Bencher, black_box};
use the_freeway_game::freeway_game;

#[bench]
fn bench(bencher: &mut Bencher) {
    let other_cars: [_; 100] = array::from_fn(|i| {
        if i.is_multiple_of(2) {
            (-5., 1.)
        } else {
            (5., 1000.)
        }
    });
    bencher.iter(|| freeway_game(black_box(100.), black_box(100.), black_box(&other_cars)));
}
