#![feature(test)]

extern crate test;
use nth_power_rules_them_all::modified_sum;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..10 {
            black_box(modified_sum(black_box(&[2, 7, 13, 17]), black_box(2)));
            black_box(modified_sum(black_box(&[1, 2, 3, 4, 5]), black_box(3)));
            black_box(modified_sum(black_box(&[2, 4, 6, 8]), black_box(6)));
        }
    });
}
