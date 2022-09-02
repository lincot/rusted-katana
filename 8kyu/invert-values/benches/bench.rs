#![feature(test)]

extern crate test;
use invert_values::invert;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let values = black_box(&[
        165, 238, 120, 186, 174, 48, 19, 194, 77, 58, 198, 72, 198, 121, 61, 103, 149, 37, 188,
        141, 47, 206, 195, 79, 82, 186, 67, 238, 129, 32, 53, 237, 250, 211, 36, 135, 227, 64, 189,
        253, 5, 173, 167, 3, 123, 98, 43, 230, 185, 163, 252, 94, 170, 88, 119, 168, 64, 134, 81,
        92, 155, 201, 199, 180, 130, 193, 244, 47, 17, 27, 181, 199, 46, 191, 228, 180, 144, 170,
        24, 72, 235, 165, 179, 240, 222, 123, 83, 205, 129, 143, 239, 155, 178, 49, 113, 223, 50,
        68, 111, 158,
    ]);
    bencher.iter(|| invert(values));
}
