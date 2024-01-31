//! <https://www.codewars.com/kata/59d7c910f703c460a2000034/train/rust>

pub mod quest {

    pub fn solomons_quest(r: Vec<(i32, i32, i32)>) -> (i32, i32) {
        let mut res = (0, 0);
        let mut level = 0;
        for (level_shift, direction, mut distance) in r {
            level += level_shift;
            distance <<= level;
            match direction {
                0 => res.1 += distance,
                1 => res.0 += distance,
                2 => res.1 -= distance,
                _ => res.0 -= distance,
            }
        }
        res
    }
}
