#![feature(test)]

extern crate test;
use row_weights::row_weights;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let array = black_box(&[
        95, 66, 98, 80, 87, 95, 73, 100, 84, 52, 76, 96, 98, 87, 76, 93, 89, 84, 95, 58, 98, 99,
        61, 54, 64, 100, 54, 64, 90, 63, 100, 59, 55, 84, 60, 74, 61, 95, 65, 80, 56, 83, 53, 89,
        87, 75, 98, 59, 90, 73, 63, 83, 94, 59, 79, 76, 66, 90, 79, 53, 70, 59, 92, 59, 79, 60, 52,
        63, 52, 85, 57, 62, 92, 61, 99, 98, 53, 61, 91, 63, 71, 50, 81, 83, 62, 56, 76, 70, 52, 69,
        51, 81, 72, 84, 71, 67, 75, 56, 97, 70,
    ]);
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(row_weights(array.to_vec()));
        }
    });
}
