#![feature(test)]

extern crate test;
use alphabetical_addition::add_letters;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let letters = black_box(vec![
        'r', 'n', 'c', 'p', 'a', 'k', 'w', 'g', 'q', 'm', 'w', 'f', 'i', 'e', 'g', 'l', 'r', 'r',
        'r', 's', 'v', 'i', 'g', 'p', 'f', 'o', 'd', 'c', 't', 'e', 'q', 'g', 'w', 'i', 't', 'b',
        't', 'q', 'f', 'i', 'e', 'y', 'z', 'f', 'c', 'z', 'g', 'w', 'i', 't', 'h', 'o', 'l', 'v',
        'k', 'h', 'q', 'y', 'w', 'l', 'c', 'a', 's', 'r', 'r', 'w', 's', 'h', 'z', 'd', 'l', 'o',
        'i', 'i', 's', 'y', 'm', 'x', 'h', 'l', 'j', 's', 'b', 'f', 'g', 's', 'z', 'p', 't', 'r',
        'y', 's', 'g', 'z', 'n', 'k', 'p', 'h', 'c', 'i',
    ]);
    bencher.iter(|| add_letters(letters.clone()));
}
