#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use volume_of_a_cuboid::get_volume_of_cuboid;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| get_volume_of_cuboid(black_box(1.), black_box(2.), black_box(2.)));
}
