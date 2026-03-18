#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use the_at_operator::evaluate;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| evaluate(black_box("155 @ -48 @ -81 @ 248".into())));
}
