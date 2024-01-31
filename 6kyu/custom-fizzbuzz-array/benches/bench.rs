#![feature(test)]

extern crate test;
use custom_fizzbuzz_array::fizz_buzz_custom_solver;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        fizz_buzz_custom_solver(
            black_box("Fizz"),
            black_box("Buzz"),
            black_box(3),
            black_box(5),
        )
    });
}
