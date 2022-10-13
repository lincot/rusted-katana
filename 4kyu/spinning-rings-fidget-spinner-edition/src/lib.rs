//! <https://www.codewars.com/kata/59b0b7cd2a00d219ab0000c5/train/rust>

#![no_std]

pub const fn spinning_rings(inner_max: u64, outer_max: u64) -> u64 {
    let (i, o) = (inner_max + 1, outer_max + 1);
    if i == 0 || o == 0 || i % 2 == 0 && i < 2 * o {
        i / 2
    } else if i <= o {
        let t = ((o - 1) / i + 1) * i + o;
        if t % 2 == 0 {
            t / 2
        } else {
            (t + i) / 2
        }
    } else if i < 2 * o {
        if (i + o) % 2 == 0 {
            (i + o) / 2
        } else {
            ((2 * i) / o - 1) * (o / 2) + i
        }
    } else {
        let t = ((i + 1) / o - 1) * o + i;
        if t % 2 == 0 {
            t / 2
        } else if o % 2 == 1 {
            (t + o) / 2
        } else {
            ((2 * i) / o - 1) * (o / 2) + i
        }
    }
}
