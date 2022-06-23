//! <https://www.codewars.com/kata/554ca54ffa7d91b236000023/train/rust>

pub fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut counts = [0; u8::MAX as usize + 1];

    lst.iter()
        .copied()
        .filter(|&x| {
            counts[x as usize] += 1;
            counts[x as usize] <= n
        })
        .collect()
}
