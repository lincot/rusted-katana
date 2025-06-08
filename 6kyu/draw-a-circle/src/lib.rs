//! <https://www.codewars.com/kata/59c804d923dacc6c41000004/train/rust>

use unchecked_std::prelude::*;

pub fn circle(radius: i32) -> String {
    if radius < 0 {
        return String::new();
    }
    if radius == 0 {
        return "\n".into();
    }
    let radius = radius as usize;

    assert!(radius <= MAX_RADIUS);
    let mut res = String::with_capacity(get_capacity(radius));

    (0..2 * radius - 1).for_each(|row| {
        let dist_to_center = row.max(radius - 1) - row.min(radius - 1);
        let half_width: usize = unsafe {
            (((radius.pow(2) - dist_to_center.pow(2)) as f64).sqrt() + 0.999_999).to_int_unchecked()
        };

        unsafe {
            for _ in 0..radius - half_width {
                res.push_unchecked(' ');
            }
            for _ in 0..2 * half_width - 1 {
                res.push_unchecked('█');
            }
            for _ in 0..radius - half_width {
                res.push_unchecked(' ');
            }
            res.push_unchecked('\n');
        }
    });

    res
}

#[allow(dead_code)]
const MAX_RADIUS_64: usize = 876_706_528;
#[allow(dead_code)]
const MAX_RADIUS_32: usize = 13377;
#[allow(dead_code)]
const MAX_RADIUS_16: usize = 52;

#[cfg(target_pointer_width = "64")]
const MAX_RADIUS: usize = MAX_RADIUS_64;
#[cfg(target_pointer_width = "32")]
const MAX_RADIUS: usize = MAX_RADIUS_32;
#[cfg(target_pointer_width = "16")]
const MAX_RADIUS: usize = MAX_RADIUS_16;

const fn get_capacity(radius: usize) -> usize {
    (2 * radius - 1).pow(2) * "█".len() + "\n".len() * (2 * radius - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_radius() {
        assert!(isize::try_from(get_capacity(MAX_RADIUS)).is_ok());
        assert!(isize::try_from(get_capacity(MAX_RADIUS + 1)).is_err());

        assert!(i64::try_from(get_capacity(MAX_RADIUS_64)).is_ok());
        assert!(i64::try_from(get_capacity(MAX_RADIUS_64 + 1)).is_err());

        assert!(i32::try_from(get_capacity(MAX_RADIUS_32)).is_ok());
        assert!(i32::try_from(get_capacity(MAX_RADIUS_32 + 1)).is_err());

        assert!(i16::try_from(get_capacity(MAX_RADIUS_16)).is_ok());
        assert!(i16::try_from(get_capacity(MAX_RADIUS_16 + 1)).is_err());
    }
}
