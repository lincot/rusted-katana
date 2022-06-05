#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box("стххфпцыйжлгчюуньрощдтзкн");
    let b = black_box("еньехаоаъцзубчмфшпащусщям");
    bencher.iter(|| solution::solve(a, b));
}
