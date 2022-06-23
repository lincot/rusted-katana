//! <https://www.codewars.com/kata/5f55ecd770692e001484af7d/train/rust>

pub fn mirror(list: &[i32]) -> Vec<i32> {
    if list.len() <= 1 {
        return list.into();
    }

    let mut res = Vec::with_capacity(2 * list.len() - 1);

    res.extend(list);
    res.sort_unstable();

    for i in (0..res.len() - 1).rev() {
        res.push(*unsafe { res.get_unchecked(i) });
    }

    res
}
