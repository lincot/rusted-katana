#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use toggle_set_and_clear_bits_bit_manipulation_basics::{
    clear_bit, clear_multiple_bits, is_bit_set, set_bit, set_multiple_bits, toggle_bit,
    toggle_multiple_bits,
};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(toggle_bit(black_box(5), black_box(1)));
        black_box(set_bit(black_box(5), black_box(1)));
        black_box(clear_bit(black_box(7), black_box(1)));
        black_box(is_bit_set(black_box(5), black_box(0)));
        black_box(set_multiple_bits(black_box(5), black_box(3)));
        black_box(clear_multiple_bits(black_box(7), black_box(2)));
        black_box(toggle_multiple_bits(black_box(5), black_box(3)));
    });
}
