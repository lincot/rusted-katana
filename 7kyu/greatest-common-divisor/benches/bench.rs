#![feature(test)]

extern crate test;
use greatest_common_divisor::gcd;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..5 {
            black_box(gcd(black_box(u32::MAX / 3), black_box(u32::MAX / 2 + 100)));
        }
    });
}
