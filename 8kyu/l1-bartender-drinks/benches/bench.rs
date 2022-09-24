#![no_std]
#![feature(test)]

extern crate test;
use l1_bartender_drinks::get_drink_by_profession;
use test::{black_box, Bencher};

#[bench]
fn bench_jabroni(bencher: &mut Bencher) {
    let param = black_box("jAbrOni");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_drink_by_profession(param));
        }
    });
}

#[bench]
fn bench_school_counselor(bencher: &mut Bencher) {
    let param = black_box("schoOl counsElor");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_drink_by_profession(param));
        }
    });
}

#[bench]
fn bench_programmer(bencher: &mut Bencher) {
    let param = black_box("pRoGrAmMeR");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_drink_by_profession(param));
        }
    });
}

#[bench]
fn bench_bike_gang_member(bencher: &mut Bencher) {
    let param = black_box("biKe ganG memBer");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_drink_by_profession(param));
        }
    });
}

#[bench]
fn bench_politician(bencher: &mut Bencher) {
    let param = black_box("polIticIan");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_drink_by_profession(param));
        }
    });
}

#[bench]
fn bench_rapper(bencher: &mut Bencher) {
    let param = black_box("raPPer");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_drink_by_profession(param));
        }
    });
}

#[bench]
fn bench_anyone(bencher: &mut Bencher) {
    let param = black_box("AnyOne");
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(get_drink_by_profession(param));
        }
    });
}
