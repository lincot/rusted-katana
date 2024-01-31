//! <https://www.codewars.com/kata/566be96bb3174e155300001b/train/rust>

pub fn max_ball(v0: i32) -> i32 {
    unsafe {
        (v0 as f64)
            .mul_add(10. / 3.6 / 9.81, 0.5)
            .to_int_unchecked()
    }
}
