//! <https://www.codewars.com/kata/55f1614853ddee8bd4000014/train/rust>

pub fn count_pairs_int(diff: u32, n_max: u32) -> u32 {
    if n_max <= diff {
        return 0;
    }
    let (diff, n_max) = (diff as usize, n_max as usize);
    let mut nds = vec![0u32; n_max];
    for i in 1..n_max {
        let mut j = i;
        while j < n_max {
            nds[j] += 1;
            j += i;
        }
    }
    nds.iter().zip(&nds[diff..]).filter(|(x, y)| x == y).count() as _
}
