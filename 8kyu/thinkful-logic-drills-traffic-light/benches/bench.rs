#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use thinkful_logic_drills_traffic_light::update_light;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for current in ["green", "yellow", "red"] {
            black_box(update_light(black_box(current)));
        }
    });
}
