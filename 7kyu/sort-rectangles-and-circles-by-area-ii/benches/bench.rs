#![no_std]
#![feature(test)]

extern crate test;
use either::Either;
use sort_rectangles_and_circles_by_area_ii::sort_by_area;
use test::{black_box, Bencher};

#[bench]
fn bench(bencher: &mut Bencher) {
    let seq = black_box(&[
        Either::Left((4.23, 6.43)),
        Either::Right(1.23),
        Either::Right(3.444),
        Either::Left((1.342, 3.212)),
        Either::Right(206.5),
        Either::Right(273.2),
        Either::Left((845.9, 399.4)),
        Either::Right(607.35),
        Either::Right(142.1),
        Either::Left((717.1, 901.5)),
        Either::Right(348.999),
        Either::Right(805.3),
        Either::Left((181., 318.1)),
        Either::Left((4.23, 6.43)),
        Either::Right(1.23),
        Either::Right(3.444),
        Either::Left((1.342, 3.212)),
        Either::Right(206.5),
        Either::Right(273.2),
        Either::Left((845.9, 399.4)),
        Either::Right(607.35),
        Either::Right(142.1),
        Either::Left((717.1, 901.5)),
        Either::Right(348.999),
        Either::Right(805.3),
        Either::Left((181., 318.1)),
    ]);
    bencher.iter(|| sort_by_area(seq));
}
