#![feature(test)]

extern crate test;
use line_safari_point_distance_from_a_line::distance_from_line;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        distance_from_line(
            black_box((10., 10.)),
            black_box((30., 10.)),
            black_box((20., 25.)),
        )
    });
}
