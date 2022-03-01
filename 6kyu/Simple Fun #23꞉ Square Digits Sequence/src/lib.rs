//! <https://www.codewars.com/kata/5886d65e427c27afeb0000c1/train/rust>

use rustc_hash::FxHashSet;

pub fn square_digits_sequence(a0: u32) -> usize {
    let mut an = a0;
    let mut seen = FxHashSet::with_capacity_and_hasher(19, Default::default());

    loop {
        seen.insert(an);

        let mut a_next = 0;
        while an != 0 {
            a_next += (an % 10).pow(2);
            an /= 10;
        }

        if seen.contains(&a_next) {
            break;
        }
        an = a_next;
    }

    seen.len() + 1
}
