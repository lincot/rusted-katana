#![feature(test)]

extern crate test;
use swapping_cards::swap_cards;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(swap_cards(black_box(77), black_box(54)));
        black_box(swap_cards(black_box(45), black_box(23)));
        black_box(swap_cards(black_box(74), black_box(81)));
        black_box(swap_cards(black_box(75), black_box(35)));
        black_box(swap_cards(black_box(21), black_box(25)));
    });
}
