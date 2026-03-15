#![feature(test)]

extern crate test;
use raise_me_to_the_third_power_search_my_divisors_dot_dot_dot_dot_dot_dot_could_you_believe_that::int_cube_sum_div;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| int_cube_sum_div(black_box(120)));
}
