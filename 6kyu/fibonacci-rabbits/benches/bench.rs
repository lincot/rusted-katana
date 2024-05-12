#![feature(test)]

extern crate test;
use fibonacci_rabbits::fib_rabbits;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(fib_rabbits(black_box(50), black_box(50)));
        black_box(fib_rabbits(black_box(40), black_box(40)));
        black_box(fib_rabbits(black_box(10), black_box(32)));
        black_box(fib_rabbits(black_box(8), black_box(12)));
    });
}
