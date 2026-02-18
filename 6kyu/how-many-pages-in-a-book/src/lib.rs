//! <https://www.codewars.com/kata/622de76d28bf330057cd6af8/train/rust>

pub const fn amount_of_pages(summary: u32) -> u32 {
    if summary >= 2889 {
        if summary <= 38889 {
            999 + (summary - 2889) / 4
        } else if summary <= 488_889 {
            9999 + (summary - 38889) / 5
        } else {
            99_999 + (summary - 488_889) / 6
        }
    } else if summary <= 9 {
        summary
    } else if summary <= 189 {
        9 + (summary - 9) / 2
    } else {
        99 + (summary - 189) / 3
    }
}
