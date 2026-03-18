#![feature(test)]

extern crate test;
use basics_09_shifting_bits_right_dot_dot_dot::previous_power_of_2;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for n in [1000, 500, -1000, -500, 0, 200_000, -200_000] {
            black_box(previous_power_of_2(black_box(n)));
        }
    });
}
