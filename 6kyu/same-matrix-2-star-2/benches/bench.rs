#![feature(test)]

extern crate test;
use same_matrix_2_star_2::count_different_matrices;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_different_matrices(black_box(&[
            [1, 2, 3, 4],
            [3, 1, 4, 2],
            [4, 3, 2, 1],
            [2, 4, 1, 3],
            [3, 1, 2, 3],
            [3, 1, 2, 3],
            [1, 3, 3, 2],
            [3, 2, 1, 3],
            [5, 1, 2, 6],
            [5, 4, 3, 5],
            [2, 5, 6, 1],
            [1, 2, 2, 1],
            [1, 1, 2, 2],
            [2, 1, 1, 2],
            [2, 1, 2, 1],
            [1, 2, 1, 2],
        ]))
    });
}
