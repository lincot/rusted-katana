#![feature(test)]

extern crate test;
use new_cashier_does_not_know_about_space_or_shift::get_order;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input = black_box("milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza");
    bencher.iter(|| get_order(input.into()));
}
