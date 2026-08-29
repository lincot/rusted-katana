//! <https://www.codewars.com/kata/5a58ca28e626c55ae000018a/train/rust>

use core::f64::consts::PI;

use libm::sin;

pub fn area_of_polygon_inside_circle(circle_radius: f64, number_of_sides: u32) -> f64 {
    number_of_sides as f64 * circle_radius.powi(2) * sin(2. * PI / number_of_sides as f64) / 2.
}
