//! <https://www.codewars.com/kata/595bbea8a930ac0b91000130/train/rust>

pub fn calculate_1_rm(w: i32, r: i32) -> i32 {
    if r == 0 {
        0
    } else if r == 1 {
        w
    } else if r < 6 {
        (r as f64).powf(0.1).mul_add(w as f64, 0.5) as _
    } else if (10..=37).contains(&r) {
        ((100 * w) as f64 / (r as f64).mul_add(-2.67123, 101.3) + 0.5) as _
    } else {
        w + (w * r + 15) / 30
    }
}
