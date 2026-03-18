#![feature(test)]

extern crate test;
use grasshopper_check_for_factor::check_for_factor;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| check_for_factor(black_box(123_123), black_box(123)));
}
