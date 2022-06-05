#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let price = black_box(12);
    let discount = black_box(50);
    let holiday_cost = black_box(1000);
    bencher.iter(|| {
        for _ in 0..1_000_000 {
            black_box(solution::duty_free(price, discount, holiday_cost));
        }
    });
}
