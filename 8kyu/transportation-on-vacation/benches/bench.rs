#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use transportation_on_vacation::rental_car_cost;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(rental_car_cost(black_box(2)));
        black_box(rental_car_cost(black_box(5)));
        black_box(rental_car_cost(black_box(50)));
    });
}
