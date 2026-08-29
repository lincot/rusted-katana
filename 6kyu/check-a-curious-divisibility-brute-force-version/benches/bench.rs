#![feature(test)]

extern crate test;
use check_a_curious_divisibility_brute_force_version::find_int;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_int(black_box(500), black_box(700), black_box(4)));
}
