#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use transportation_on_vacation::rental_car_cost;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| rental_car_cost(black_box(5)));
}
