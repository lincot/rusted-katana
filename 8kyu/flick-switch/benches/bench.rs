#![no_std]
#![feature(test)]

extern crate test;
use flick_switch::flick_switch;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        for _ in 0..1000 {
            black_box(flick_switch(black_box(&[
                "bicycle",
                "jarmony",
                "flick",
                "sheep",
                "flick",
                "codewars",
                "flick",
                "code",
                "wars",
                "flick",
                "chocolate",
                "adventure",
                "sunshine",
            ])));
        }
    });
}
