#![feature(test)]

extern crate test;
use simple_fun_number_11_swap_adjacent_bits::swap_adjacent_bits;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| swap_adjacent_bits(black_box(83748)));
}
