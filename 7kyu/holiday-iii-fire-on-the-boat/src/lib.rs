//! <https://www.codewars.com/kata/57e8fba2f11c647abc000944/train/rust>

use my_prelude::prelude::*;

pub fn fire_fight(s: &str) -> String {
    const FIRE: &str = "Fire";
    const WATER: &str = "~~";
    assert!(WATER.len() < FIRE.len());

    let mut res = String::with_capacity(s.len());
    let mut i = 0;
    while i < s.len().saturating_sub(FIRE.len() - 1) {
        unsafe {
            if s.get_unchecked(i..).starts_with(FIRE) {
                res.push_str_unchecked(WATER);
                i += FIRE.len();
            } else {
                res.as_mut_vec()
                    .push_unchecked(*s.as_bytes().get_unchecked(i));
                i += 1;
            }
        }
    }
    unsafe {
        res.push_str_unchecked(s.get_unchecked(i..));
        res
    }
}
