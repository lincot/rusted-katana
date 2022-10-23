//! <https://www.codewars.com/kata/530265044b7e23379d00076a/train/rust>

#![no_std]
#![feature(core_intrinsics)]

use core::intrinsics::fmaf32;

type Point = (f32, f32);
type Line = (Point, Point);

fn intersects(l1: Line, l2: Line) -> bool {
    let p = |a: Point, b: Point, c: Point| unsafe {
        fmaf32(b.1 - a.1, c.0 - b.0, -(b.0 - a.0) * (c.1 - b.1))
    };
    p(l1.0, l1.1, l2.0) * p(l1.0, l1.1, l2.1) < 0. && p(l2.0, l2.1, l1.0) * p(l2.0, l2.1, l1.1) < 0.
}

pub fn point_in_poly(poly: &[Point], point: Point) -> bool {
    assert!(poly.len() >= 3);
    let line = (point, (8., point.1));
    let side = (poly[poly.len() - 1], poly[0]);
    let mut intersections = intersects(side, line) as usize;
    for i in 1..poly.len() {
        let side = (poly[i - 1], poly[i]);
        if intersects(side, line) {
            intersections += 1;
        }
    }
    intersections % 2 == 1
}
