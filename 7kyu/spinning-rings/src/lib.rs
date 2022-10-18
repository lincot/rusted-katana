//! <https://www.codewars.com/kata/59afff65f1c8274f270020f5/train/rust>

#![no_std]

pub const fn spinning_rings(inner_max: u8, outer_max: u8) -> u8 {
    let (i, o) = (inner_max + 1, outer_max + 1);
    if i % 2 == 0 && i < 2 * o {
        i / 2
    } else if i <= o {
        let t = (2 * o - (o - 1) % i - 1) as u16 + i as u16;
        if t % 2 == 0 {
            (t / 2) as _
        } else {
            (t / 2) as u8 + i / 2 + 1
        }
    } else if i < 2 * o {
        if (i + o) % 2 == 0 {
            (i + o) / 2
        } else {
            ((2 * i) / o - 1) * (o / 2) + i
        }
    } else {
        let t = 2 * i - o - (i + 1) % o + 1;
        if t % 2 == 0 {
            t / 2
        } else if o % 2 == 1 {
            t / 2 + o / 2 + 1
        } else {
            ((2 * i) / o - 1) * (o / 2) + i
        }
    }
}
