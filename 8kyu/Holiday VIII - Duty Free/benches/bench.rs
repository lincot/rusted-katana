#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const PRICE: i32 = 12;
const DISCOUNT: i32 = 50;
const HOLIDAY_COST: i32 = 1000;

#[bench]
fn bench(b: &mut Bencher) {
    let price = black_box(PRICE);
    let discount = black_box(DISCOUNT);
    let holiday_cost = black_box(HOLIDAY_COST);

    b.iter(|| {
        for _ in 0..1_000_000 {
            black_box(solution::duty_free(price, discount, holiday_cost));
        }
    })
}
