#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use was_the_package_received_before_it_was_sent_simplified::was_package_received_yesterday;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        was_package_received_yesterday(black_box(-11), black_box(-8), black_box(3), black_box(12))
    });
}
