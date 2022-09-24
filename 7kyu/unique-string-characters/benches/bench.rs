#![no_std]
#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use unique_string_characters::solve;

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box("стххфпцыйжлгчюуньрощдтзкн");
    let b = black_box("еньехаоаъцзубчмфшпащусщям");
    bencher.iter(|| solve(a, b));
}
