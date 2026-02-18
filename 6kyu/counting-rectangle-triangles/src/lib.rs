//! <https://www.codewars.com/kata/57d99f6bbfcdc5b3b0000286/train/rust>

pub fn count_right_triangles(points: &[(i32, i32)]) -> u32 {
    let mut points = points.to_vec();
    points.sort_unstable();
    points.dedup();

    let mut res = 0;
    for (i, &p1) in points.iter().enumerate() {
        for (j, &p2) in (i + 1..).zip(&points[i + 1..]) {
            for &p3 in &points[j + 1..] {
                if is_right_triangle(p1, p2, p3) {
                    res += 1;
                }
            }
        }
    }
    res
}

const fn is_right_triangle(a: (i32, i32), b: (i32, i32), c: (i32, i32)) -> bool {
    let ab = (b.0 - a.0, b.1 - a.1);
    let bc = (c.0 - b.0, c.1 - b.1);
    let ca = (a.0 - c.0, a.1 - c.1);
    let ab_bc = ab.0 * bc.0 + ab.1 * bc.1;
    let bc_ca = bc.0 * ca.0 + bc.1 * ca.1;
    let ca_ab = ca.0 * ab.0 + ca.1 * ab.1;
    ab_bc == 0 || bc_ca == 0 || ca_ab == 0
}
