//! <https://www.codewars.com/kata/559cc2d2b802a5c94700000c/train/rust>

pub fn consecutive(xs: &[i16]) -> i16 {
    let mut it = xs.iter();

    if let Some(&first) = it.next() {
        let (min, max) = it.fold((first, first), |(min, max), &new| {
            (min.min(new), max.max(new))
        });

        max - min + 1 - xs.len() as i16
    } else {
        0
    }
}
