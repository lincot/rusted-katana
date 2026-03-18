#![feature(test)]

extern crate test;
use dollars_and_cents::format_money;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| format_money(black_box(314.16)));
}
