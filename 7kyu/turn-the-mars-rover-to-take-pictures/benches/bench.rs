#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use turn_the_mars_rover_to_take_pictures::turn;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| turn(black_box('S'), black_box('E')));
}
