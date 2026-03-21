//! <https://www.codewars.com/kata/59b0b7cd2a00d219ab0000c5/train/rust>

#[expect(clippy::manual_is_multiple_of)]
pub const fn spinning_rings(inner_max: u64, outer_max: u64) -> u64 {
    let i = inner_max + 1;
    let o = outer_max + 1;

    if i > o {
        if i % 2 == 1 && o % 2 == 0 {
            2 * i - (o + (2 * i) % o) / 2
        } else if i % 2 == 0 && o % 2 == 0 {
            i - (o + i % o) / 2
        } else if i % o % 2 == 0 {
            i - i % o / 2
        } else {
            i - (o + i % o) / 2
        }
    } else if i % 2 == 0 {
        i / 2
    } else if o % i == 0 {
        o
    } else if o % i % 2 == 0 {
        o + i - o % i / 2
    } else {
        o + (i - o % i) / 2
    }
}
