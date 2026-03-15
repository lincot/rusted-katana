#![feature(test)]

extern crate test;
use exclamation_marks_series_number_7_remove_words_from_the_sentence_if_it_contains_one_exclamation_mark::remove;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| remove(black_box("Hi! !Hi! Hi! Hi")));
}
