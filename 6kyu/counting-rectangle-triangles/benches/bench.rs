#![feature(test)]

extern crate test;
use counting_rectangle_triangles::count_right_triangles;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        count_right_triangles(black_box(&[
            (30, 26),
            (36, 6),
            (12, 27),
            (9, 8),
            (9, 22),
            (6, 35),
            (26, 40),
            (35, 18),
            (27, 2),
            (19, 18),
            (2, 41),
            (18, 3),
            (4, 37),
            (13, 25),
            (21, 34),
            (27, 45),
            (26, 12),
            (23, 16),
            (28, 1),
            (0, 25),
            (12, 25),
            (10, 41),
            (24, 18),
            (31, 38),
            (28, 17),
            (9, 23),
            (29, 1),
            (21, 43),
            (20, 46),
            (50, 10),
        ]))
    });
}
