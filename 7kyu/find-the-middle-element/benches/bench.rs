#![no_std]
#![feature(test)]

extern crate test;
use find_the_middle_element::gimme;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let input_array = black_box([1, 2, 3]);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(gimme(input_array));
        }
    });
}
