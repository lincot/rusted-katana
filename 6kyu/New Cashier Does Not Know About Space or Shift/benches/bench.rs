#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input = black_box("milkshakepizzachickenfriescokeburgerpizzasandwichmilkshakepizza");
    bencher.iter(|| solution::get_order(input.into()))
}
