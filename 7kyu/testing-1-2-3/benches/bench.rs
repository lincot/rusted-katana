#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use testing_1_2_3::number;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        number(black_box(&[
            "#![no_std]",
            "",
            "fn main() {",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "    let _ = 0;",
            "}",
        ]))
    });
}
