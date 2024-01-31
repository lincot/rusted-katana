#![feature(test)]

extern crate test;
use test::{black_box, Bencher};
use unique_string_characters::solve;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        solve(
            black_box("стххфпцыйжлгчюуньрощдтзкн"),
            black_box("еньехаоаъцзубчмфшпащусщям"),
        )
    });
}
