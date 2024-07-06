#![feature(test)]

extern crate test;
use program_a_calculator_number_2_3d_vectors::Vector;
use test::{black_box, Bencher};

#[bench]
fn bench_new(bencher: &mut Bencher) {
    bencher.iter(|| Vector::new(black_box(1.0), black_box(2.0), black_box(3.0)));
}

#[bench]
fn bench_get_magnitude(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Vector::new(1.0, 2.0, 3.0)).get_magnitude());
}

#[bench]
fn bench_multiply_by_scalar(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Vector::new(1.0, 2.0, 3.0)).multiply_by_scalar(black_box(2.0)));
}

#[bench]
fn bench_dot(bencher: &mut Bencher) {
    bencher
        .iter(|| black_box(Vector::new(1.0, 2.0, 3.0)).dot(black_box(Vector::new(4.0, 5.0, 6.0))));
}

#[bench]
fn bench_cross(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Vector::new(1.0, 2.0, 3.0)).cross(black_box(Vector::new(4.0, 5.0, 6.0)))
    });
}

#[bench]
fn bench_is_parallel_to(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Vector::new(1.0, 2.0, 3.0)).is_parallel_to(black_box(Vector::new(2.0, 4.0, 6.0)))
    });
}

#[bench]
fn bench_is_perpendicular_to(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(Vector::new(1.0, 0.0, 0.0))
            .is_perpendicular_to(black_box(Vector::new(0.0, 1.0, 0.0)))
    });
}

#[bench]
fn bench_normalize(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Vector::new(1.0, 2.0, 3.0)).normalize());
}

#[bench]
fn bench_is_normalized(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Vector::new(1.0, 0.0, 0.0)).is_normalized());
}

#[bench]
fn bench_add(bencher: &mut Bencher) {
    bencher.iter(|| black_box(Vector::new(1.0, 2.0, 3.0)) + black_box(Vector::new(4.0, 5.0, 6.0)));
}
