#![feature(test)]

extern crate test;
use number_paper_and_pencil_2_finding_the_formula_of_a_single_x_2_sequence_and_the_next_terms::quadratic_formula;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| quadratic_formula(black_box(-4), black_box(0), black_box(6)));
}
