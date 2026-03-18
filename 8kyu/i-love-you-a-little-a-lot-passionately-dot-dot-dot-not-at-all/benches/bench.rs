#![feature(test)]

extern crate test;
use i_love_you_a_little_a_lot_passionately_dot_dot_dot_not_at_all::how_much_i_love_you;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for nb_petals in 12..18 {
            black_box(how_much_i_love_you(black_box(nb_petals)));
        }
    });
}
