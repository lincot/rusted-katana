#![feature(test)]

extern crate test;
use find_the_first_number_in_having_a_certain_number_of_divisors_i::find_min_num;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_min_num(black_box(52)));
}
