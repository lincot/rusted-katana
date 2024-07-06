#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use transportation_on_vacation::rental_car_cost;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(rental_car_cost(black_box(5)));
        }
    });
}
