#![feature(test)]

extern crate test;
use calculate_the_area_of_a_regular_n_sides_polygon_inside_a_circle_of_radius_r::area_of_polygon_inside_circle;
use test::{Bencher, black_box};

#[bench]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| area_of_polygon_inside_circle(black_box(3.), black_box(3)));
}
