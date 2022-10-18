#![no_std]
#![feature(test)]

extern crate test;
use replace_all_items::replace_all;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        replace_all(
            black_box(&[
                "ooh", "la", "la", "oooooh", "lah", "lol", "this", "is", "so", "gooood",
            ]),
            black_box("la"),
            black_box("baby"),
        )
    });
}
