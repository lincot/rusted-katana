#![no_std]
#![feature(test)]

extern crate test;
use l1_bartender_drinks::get_drink_by_profession;
use test::{black_box, Bencher};

#[bench]
fn bench_jabroni(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(get_drink_by_profession(black_box("jAbrOni")));
        }
    });
}

#[bench]
fn bench_school_counselor(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(get_drink_by_profession(black_box("schoOl counsElor")));
        }
    });
}

#[bench]
fn bench_programmer(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(get_drink_by_profession(black_box("pRoGrAmMeR")));
        }
    });
}

#[bench]
fn bench_bike_gang_member(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(get_drink_by_profession(black_box("biKe ganG memBer")));
        }
    });
}

#[bench]
fn bench_politician(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(get_drink_by_profession(black_box("polIticIan")));
        }
    });
}

#[bench]
fn bench_rapper(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(get_drink_by_profession(black_box("raPPer")));
        }
    });
}

#[bench]
fn bench_anyone(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..if cfg!(miri) { 1 } else { 1000 } {
            black_box(get_drink_by_profession(black_box("AnyOne")));
        }
    });
}
