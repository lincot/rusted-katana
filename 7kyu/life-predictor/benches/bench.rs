#![feature(test)]

extern crate test;
use chrono::{NaiveDate, TimeDelta};
use life_predictor::life_predictor;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut date = NaiveDate::from_ymd_opt(1999, 1, 1).unwrap();
        for _ in 0..if cfg!(miri) { 5 } else { 365 } {
            black_box(life_predictor(black_box(&date.to_string())));
            date += TimeDelta::try_days(1).unwrap();
        }
    });
}
