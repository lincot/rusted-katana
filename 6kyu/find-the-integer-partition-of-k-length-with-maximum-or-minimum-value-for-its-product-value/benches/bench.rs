#![feature(test)]

extern crate test;
use find_the_integer_partition_of_k_length_with_maximum_or_minimum_value_for_its_product_value::find_spec_partition;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| find_spec_partition(black_box(1005), black_box(30), black_box("max")));
}
