#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

const A: &str = "стххфпцыйжлгчюуньрощдтзкн";
const B: &str = "еньехаоаъцзубчмфшпащусщям";

#[bench]
fn bench(bencher: &mut Bencher) {
    let a = black_box(A);
    let b = black_box(B);

    bencher.iter(|| solution::solve(a, b))
}
