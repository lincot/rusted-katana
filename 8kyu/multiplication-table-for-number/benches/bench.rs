#![feature(test)]

extern crate test;
use multiplication_table_for_number::multi_table;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| multi_table(black_box(5)));
}
