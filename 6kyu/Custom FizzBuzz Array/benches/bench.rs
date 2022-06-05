#![feature(test)]

extern crate test;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let string_one = black_box("Fizz");
    let string_two = black_box("Buzz");
    let num_one = black_box(3);
    let num_two = black_box(5);
    bencher.iter(|| solution::fizz_buzz_custom_solver(string_one, string_two, num_one, num_two));
}
