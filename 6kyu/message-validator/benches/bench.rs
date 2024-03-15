#![feature(test)]

extern crate test;
use message_validator::is_valid_message;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        is_valid_message(black_box(
            "4code13hellocodewars1a2bb3ccc4dddd5eeeee3hey5hello2hi",
        ))
    });
}
