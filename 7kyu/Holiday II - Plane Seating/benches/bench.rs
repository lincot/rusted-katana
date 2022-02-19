#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench_valid(bencher: &mut Bencher) {
    let seat_number = black_box("20B");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::plane_seat(seat_number));
        }
    })
}

#[bench]
fn bench_invalid(bencher: &mut Bencher) {
    let seat_number = black_box("58I");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::plane_seat(seat_number));
        }
    })
}
