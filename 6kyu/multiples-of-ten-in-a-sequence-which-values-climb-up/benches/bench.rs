#![feature(test)]

extern crate test;
use multiples_of_ten_in_a_sequence_which_values_climb_up::find_multiples_of_10_sf;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_multiples_of_10_sf(black_box(300)));
}
