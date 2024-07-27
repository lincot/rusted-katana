#![feature(test)]

extern crate test;
use an_osha_approved_ladder::is_ladder_safe;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(is_ladder_safe(black_box(&[
            "#   #", "#####", "#   #", "#   #", "#####", "#   #", "#   #", "#####", "#   #",
        ])));
        black_box(is_ladder_safe(black_box(&[
            "#       #",
            "#########",
            "#########",
            "#########",
            "#########",
            "#########",
            "#########",
            "#########",
            "#       #",
        ])));
        black_box(is_ladder_safe(black_box(&[
            "#       #",
            "#########",
            "#       #",
            "#########",
            "#       #",
            "#########",
            "#       #",
            "#########",
            "#       #",
        ])));
        black_box(is_ladder_safe(black_box(&[
            "#   #", "#  ##", "#   #", "#   #", "#####", "#   #", "#   #", "#####", "#   #",
        ])));
    });
}
