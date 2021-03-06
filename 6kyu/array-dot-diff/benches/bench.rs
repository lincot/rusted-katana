#![feature(test)]

extern crate test;
use array_dot_diff::array_diff;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(&[
        42, 8, 7, 55, 20, 37, 53, 96, 55, 73, 27, 94, 53, 9, 45, 51, 41, 72, 8, 87, 68, 50, 27, 44,
        75, 95, 92, 75, 34, 87, 16, 9, 76, 88, 85, 14, 90, 0, 95, 57, 95, 55, 50, 80, 60, 55, 45,
        73, 95, 79, 75, 92, 96, 84, 9, 64, 23, 22, 26, 11, 69, 93, 84, 8, 60, 80, 55, 35, 44, 71,
        98, 53, 60, 33, 99, 80, 31, 97, 54, 99, 14, 63, 96, 82, 67, 13, 36, 16, 35, 24, 23, 88, 30,
        61, 1, 94, 51, 12, 96, 19,
    ]);
    let b = black_box(&[
        87, 90, 21, 92, 53, 1, 8, 38, 39, 1, 94, 38, 88, 64, 69, 33, 42, 82, 32, 95, 7, 46, 64, 22,
        38, 54, 65, 29, 37, 39, 51, 78, 29, 73, 23, 44, 72, 50, 43, 83, 50, 52, 75, 49, 76, 94, 46,
        64, 87, 5, 51, 69, 46, 1, 49, 92, 43, 90, 28, 27, 19, 23, 75, 20, 16, 60, 95, 93, 27, 88,
        77, 9, 17, 12, 23, 13, 13, 3, 40, 79, 38, 68, 13, 39, 10, 77, 90, 81, 67, 4, 53, 33, 99,
        42, 20, 61, 78, 82, 44, 16,
    ]);
    bencher.iter(|| array_diff(a.to_vec(), b.to_vec()));
}
