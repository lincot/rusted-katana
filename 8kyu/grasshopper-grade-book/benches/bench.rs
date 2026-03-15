#![feature(test)]

extern crate test;
use grasshopper_grade_book::get_grade;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(get_grade(black_box(50), black_box(55), black_box(60)));
        black_box(get_grade(black_box(60), black_box(65), black_box(70)));
        black_box(get_grade(black_box(70), black_box(75), black_box(80)));
        black_box(get_grade(black_box(80), black_box(85), black_box(90)));
        black_box(get_grade(black_box(90), black_box(95), black_box(100)));
    });
}
