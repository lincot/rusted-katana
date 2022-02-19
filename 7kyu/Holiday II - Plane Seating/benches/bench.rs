#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const SEAT_NUMBER_VALID: &str = "20B";
const SEAT_NUMBER_INVALID: &str = "58I";

#[bench]
fn bench_valid(bencher: &mut Bencher) {
    let seat_number = black_box(SEAT_NUMBER_VALID);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::plane_seat(seat_number));
        }
    })
}

#[bench]
fn bench_invalid(bencher: &mut Bencher) {
    let seat_number = black_box(SEAT_NUMBER_INVALID);

    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(solution::plane_seat(seat_number));
        }
    })
}
