//! <https://www.codewars.com/kata/5376b901424ed4f8c20002b7/train/rust>

use core::cmp::Ordering;

pub fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    #[allow(clippy::suboptimal_flops)]
    let dist_squared = |a: (f64, f64), b: (f64, f64)| (a.0 - b.0).powi(2) + (a.1 - b.1).powi(2);

    let mut points = points.to_vec().into_boxed_slice();
    points.sort_unstable_by(|a, b| {
        if a < b {
            Ordering::Less
        } else if a > b {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    let mut min_dist = f64::MAX;
    let mut best_pair = ((0., 0.), (0., 0.));

    let mut i = 0;
    while i < points.len() {
        let a = points[i];
        for &b in &points[i + 1..] {
            let dist = dist_squared(a, b);
            if dist < min_dist {
                min_dist = dist;
                best_pair = (a, b);
            } else if (a.0 - b.0).powi(2) > min_dist {
                break;
            }
        }

        i += 1;
    }

    best_pair
}
