//! <https://www.codewars.com/kata/56b97b776ffcea598a0006f2/train/rust>

pub fn bubblesort_once(lst: &[u32]) -> Vec<u32> {
    let Some(mut max) = lst.first().copied() else {
        return Vec::new();
    };
    lst[1..]
        .iter()
        .chain([&u32::MAX])
        .map(|&x| {
            if x > max {
                let r = max;
                max = x;
                r
            } else {
                x
            }
        })
        .collect()
}
