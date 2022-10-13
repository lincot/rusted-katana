#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use testing_1_2_3::number;

#[bench]
fn bench(bencher: &mut Bencher) {
    let lines = black_box(&[
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
    ]);
    bencher.iter(|| number(lines));
}
