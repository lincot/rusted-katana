#![no_std]
#![feature(test)]

extern crate test;
use going_to_the_cinema::movie;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let card = black_box(500);
    let ticket = black_box(15);
    let perc = black_box(0.9);
    bencher.iter(|| movie(card, ticket, perc));
}
