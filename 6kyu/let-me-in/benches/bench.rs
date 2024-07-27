#![feature(test)]

extern crate test;
use let_me_in::get_in_line;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(get_in_line(black_box(&[0, 8, 2, 1, 4, 2, 12, 3, 2])));
        black_box(get_in_line(black_box(&[
            2, 3, 1, 4, 5, 2, 1, 0, 8, 5, 6, 1,
        ])));
        black_box(get_in_line(black_box(&[12, 3, 19, 14, 1, 19, 16, 4, 0, 1])));
        black_box(get_in_line(black_box(&[13, 20, 3, 3, 14, 5, 13, 0, 8, 5])));
        black_box(get_in_line(black_box(&[16, 4, 3, 0, 1, 3, 7, 3, 10, 1])));
        black_box(get_in_line(black_box(&[1, 1, 1, 3, 3, 8, 3, 14, 3, 0])));
    });
}
