#![feature(test)]

extern crate test;
use product_array_array_series_number_5::product_array;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let arr = black_box(&[18, 9, 3, 9, 8, 10, 5, 16, 9, 14]);
    bencher.iter(|| product_array(arr));
}
