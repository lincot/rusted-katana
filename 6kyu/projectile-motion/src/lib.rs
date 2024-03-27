//! <https://www.codewars.com/kata/5af96cea3e9715ec670001dd/train/rust>

use digital::{MaxLenBase10, WriteNumUnchecked};
use libm::{cos, sin};
use unchecked_core::{PushStrUnchecked, PushUnchecked};

pub struct Projectile {
    height: u64,
    vertical_velocity: f64,
    horizontal_velocity: f64,
}

unsafe fn write_with_up_to_three_fractional_digits(res: &mut String, n: f64) {
    let n = (n * 1000.).round() as u64;
    res.write_num_unchecked(n / 1000, 10, false, false);
    let d0 = b'0' + (n % 1000 / 100) as u8;
    let d1 = b'0' + (n % 100 / 10) as u8;
    let d2 = b'0' + (n % 10) as u8;
    res.as_mut_vec().push_unchecked(b'.');
    res.as_mut_vec().push_unchecked(d0);
    if d1 != b'0' || d2 != b'0' {
        res.as_mut_vec().push_unchecked(d1);
    }
    if d2 != b'0' {
        res.as_mut_vec().push_unchecked(d2);
    }
}

fn round_to_three_fractional_digits(n: f64) -> f64 {
    (n * 1000.).round() / 1000.
}

impl Projectile {
    pub fn new(height: u64, velocity: u64, angle: u64) -> Self {
        let angle_rad = (angle as f64).to_radians();
        Self {
            height,
            vertical_velocity: velocity as f64 * sin(angle_rad),
            horizontal_velocity: velocity as f64 * cos(angle_rad),
        }
    }

    pub fn height_eq(&self) -> String {
        const S0: &str = "h(t) = -16.0t^2 + ";
        const S1: &str = " + ";
        const S2: &str = ".0";
        let mut res = String::with_capacity(
            S0.len() + u64::MAX_LEN_BASE10 + 4 + 1 + S1.len() + u64::MAX_LEN_BASE10 + S2.len(),
        );
        unsafe {
            res.push_str_unchecked(S0);
            write_with_up_to_three_fractional_digits(&mut res, self.vertical_velocity);
            res.as_mut_vec().push_unchecked(b't');
            if self.height != 0 {
                res.push_str_unchecked(S1);
                res.write_num_unchecked(self.height, 10, false, false);
                res.push_str_unchecked(S2);
            }
        }
        res
    }

    pub fn horiz_eq(&self) -> String {
        const S0: &str = "x(t) = ";
        let mut res = String::with_capacity(S0.len() + u64::MAX_LEN_BASE10 + 4 + 1);
        unsafe {
            res.push_str_unchecked(S0);
            write_with_up_to_three_fractional_digits(&mut res, self.horizontal_velocity);
            res.as_mut_vec().push_unchecked(b't');
        }
        res
    }

    pub fn height(&self, t: f64) -> f64 {
        round_to_three_fractional_digits(
            (-16. * t).mul_add(t, self.vertical_velocity * t) + self.height as f64,
        )
    }

    pub fn horiz(&self, t: f64) -> f64 {
        round_to_three_fractional_digits(self.horizontal_velocity * t)
    }

    pub fn landing(&self) -> [f64; 3] {
        let secs = (64f64
            .mul_add(
                self.height as f64,
                self.vertical_velocity * self.vertical_velocity,
            )
            .sqrt()
            + self.vertical_velocity)
            / 32.;
        [
            round_to_three_fractional_digits(self.horiz(secs)),
            0.,
            round_to_three_fractional_digits(secs),
        ]
    }
}
