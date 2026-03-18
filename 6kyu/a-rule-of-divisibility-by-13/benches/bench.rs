#![feature(test)]

extern crate test;
use a_rule_of_divisibility_by_13::thirt;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| thirt(black_box(85_299_258)));
}
