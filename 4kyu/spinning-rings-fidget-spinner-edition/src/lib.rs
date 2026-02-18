//! <https://www.codewars.com/kata/59b0b7cd2a00d219ab0000c5/train/rust>

pub const fn spinning_rings(inner_max: u64, outer_max: u64) -> u64 {
    let (i, o) = (inner_max + 1, outer_max + 1);
    if i.is_multiple_of(2) && i < 2 * o {
        i / 2
    } else if i <= o {
        let t = 2 * o + i - (o - 1) % i - 1;
        if t.is_multiple_of(2) {
            t / 2
        } else {
            t / 2 + i / 2 + 1
        }
    } else if i < 2 * o {
        if (i + o).is_multiple_of(2) {
            (i + o) / 2
        } else {
            ((2 * i) / o - 1) * (o / 2) + i
        }
    } else {
        let t = 2 * i - o - (i + 1) % o + 1;
        if t.is_multiple_of(2) {
            t / 2
        } else if !o.is_multiple_of(2) {
            t / 2 + o / 2 + 1
        } else {
            ((2 * i) / o - 1) * (o / 2) + i
        }
    }
}
