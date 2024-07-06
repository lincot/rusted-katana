#![feature(test)]

extern crate test;
use find_a_very_special_set_of_numbers_in_a_certain_range::find_us;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        find_us(
            black_box(30),
            black_box(400),
            black_box(18),
            black_box(&[2, 3, 7]),
            black_box(&[6, 2, 5]),
        )
    });
}
