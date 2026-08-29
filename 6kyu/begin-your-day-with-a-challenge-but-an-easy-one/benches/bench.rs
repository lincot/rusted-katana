#![feature(test)]

extern crate test;
use begin_your_day_with_a_challenge_but_an_easy_one::one_two_three;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| one_two_three(black_box(236)));
}
