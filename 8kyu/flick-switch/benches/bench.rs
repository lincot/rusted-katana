#![feature(test)]

extern crate test;
use flick_switch::flick_switch;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        flick_switch(black_box(&[
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
        ]))
    });
}
