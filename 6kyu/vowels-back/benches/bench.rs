#![feature(test)]

extern crate test;
use test::{Bencher, black_box};
use vowels_back::vowel_back;

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        vowel_back(black_box(
            "exampletestherecodewarstestcasethealphabetoriginatedaroundthe",
        ))
    });
}
