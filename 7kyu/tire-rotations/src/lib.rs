//! <https://www.codewars.com/kata/6a3ce10b3ac2038a3741c9b8/train/rust>

use core::f64::consts::PI;

pub fn tire_rotations(tire_size: &str, distance_km: i32) -> f64 {
    const MM_TO_CIRCUMFERENCE_KM: f64 = PI / 1_000_000.;

    let tire_size = tire_size.as_bytes();

    let mut tire_width = 0;
    let mut tire_width_len = 0;
    while tire_width_len < tire_size.len() {
        let b = tire_size[tire_width_len];
        if b == b'/' {
            break;
        }

        tire_width *= 10;
        tire_width += (b - b'0') as u32;
        tire_width_len += 1;
    }

    assert!(tire_size.len() >= tire_width_len + 3);

    let aspect_ratio = 10 * (*unsafe { tire_size.get_unchecked(tire_width_len + 1) } - b'0') as u32
        + (*unsafe { tire_size.get_unchecked(tire_width_len + 2) } - b'0') as u32;

    let rim_diameter = 10
        * (*unsafe { tire_size.get_unchecked(tire_size.len() - 2) } - b'0') as u32
        + (*unsafe { tire_size.get_unchecked(tire_size.len() - 1) } - b'0') as u32;

    let rim_circumference = MM_TO_CIRCUMFERENCE_KM * 25.4 * rim_diameter as f64;

    let sidewalss_height = (tire_width * aspect_ratio) as f64;
    let sidewalls_circumference = MM_TO_CIRCUMFERENCE_KM * 2. / 100. * sidewalss_height;

    distance_km as f64 / (rim_circumference + sidewalls_circumference)
}
