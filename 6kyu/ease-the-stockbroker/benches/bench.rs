#![feature(test)]

extern crate test;
use ease_the_stockbroker::balance_statement;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    const LST: &str = "GOOG 300 542.0 B, AAPL 50 145.0 B, CSCO 250.0 29 B, GOOG 200 580.0 S, GOOG 90 160.45 B, JPMC 67 12.8 S, MYSPACE 24.0 210 B, CITI 50 450 B, CSCO 100 55.5 S, ZNGA 1300 2.66 B, CLH15.NYM 50 56.32 B, OWW 1000 11.623 B, OGG 20 580.1 B";

    bencher.iter(|| balance_statement(black_box(LST)));
}
