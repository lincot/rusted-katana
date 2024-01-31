//! <https://www.codewars.com/kata/64c7bbad0a2a00198657013d/train/rust>

pub fn rev_sub(xs: &[i32]) -> Vec<i32> {
    let mut res = xs.to_vec();
    let mut i = 0;
    while let Some(Some(even_pos)) = res
        .get(i..)
        .map(|s| s.iter().position(|x| x % 2 == 0).map(|p| p + i))
    {
        let odd_pos = unsafe { res.get_unchecked(even_pos..) }
            .iter()
            .position(|x| x % 2 != 0)
            .map_or(res.len(), |p| p + even_pos);
        unsafe { res.get_unchecked_mut(even_pos..odd_pos) }.reverse();
        i = odd_pos + 1;
    }
    res
}
