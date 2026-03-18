#![feature(test)]

extern crate test;
use a_wolf_in_sheeps_clothing::warn_the_sheep;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        warn_the_sheep(black_box(&[
            "sheep", "sheep", "sheep", "wolf", "sheep", "sheep", "sheep",
        ]))
    });
}
