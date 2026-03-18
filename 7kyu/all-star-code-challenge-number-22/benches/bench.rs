#![feature(test)]

extern crate test;
use all_star_code_challenge_number_22::to_time;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| to_time(black_box(323_500)));
}
