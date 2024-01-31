//! <https://www.codewars.com/kata/5436f26c4e3d6c40e5000282/train/rust>

pub fn sum_of_n(n: i32) -> Vec<i32> {
    let len = if n > 0 { n + 1 } else { 1 - n };
    if n > 0 {
        (0..len as usize)
            .map(|i| {
                let i = i as i32;
                (i * i + i) / 2
            })
            .collect()
    } else {
        (0..len as usize)
            .map(|i| {
                let i = i as i32;
                -i * (i + 1) / 2
            })
            .collect()
    }
}
