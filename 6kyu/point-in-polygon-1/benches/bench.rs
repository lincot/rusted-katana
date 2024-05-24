#![feature(test)]

extern crate test;
use core::array;
use point_in_polygon_1::point_in_poly;
use rand::Rng;
use rand_pcg::Pcg64Mcg;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    let mut min = (f32::INFINITY, 0.);
    let mut max = (0., 0.);
    let poly: [_; 20] = array::from_fn(|i| {
        let r = rng.gen::<f32>().mul_add(2., 3.);
        let t = rng.gen::<f32>().mul_add(0.04, i as f32 / 20.);
        if r < min.0 {
            min = (r, t);
        }
        if r > max.0 {
            max = (r, t);
        }
        let t = t * core::f32::consts::TAU;
        (r * t.cos(), r * t.sin())
    });
    bencher.iter(|| point_in_poly(black_box(&poly), black_box((1., 1.))));
}
