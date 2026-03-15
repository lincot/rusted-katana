//! <https://www.codewars.com/kata/530265044b7e23379d00076a/train/rust>

pub type Point = (f32, f32);

pub fn point_in_poly(poly: &[Point], point: Point) -> bool {
    let mut res = intersects(*poly.first().unwrap(), *poly.last().unwrap(), point);
    for i in 1..poly.len() {
        if intersects(poly[i - 1], poly[i], point) {
            res = !res;
        }
    }
    res
}

fn intersects((x1, y1): Point, (x2, y2): Point, (x3, y3): Point) -> bool {
    (y1 > y3) != (y2 > y3) && (x3 < (x2 - x1) * (y3 - y1) / (y2 - y1) + x1)
}
