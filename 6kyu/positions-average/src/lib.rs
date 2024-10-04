//! <https://www.codewars.com/kata/59f4a0acbee84576800000af/train/rust>

pub fn pos_average(s: &str) -> f64 {
    let s = s.as_bytes();

    let substring_len = s.iter().position(|&b| b == b',').unwrap();
    let n_substrings = (s.len() + 2) / (substring_len + 2);
    assert!((s.len() + 2) % (substring_len + 2) == 0);

    let mut digit_counts = vec![[0isize; 10]; substring_len];

    let mut i_start = 0;
    while i_start < s.len() {
        #[allow(clippy::needless_range_loop)]
        for i in 0..substring_len {
            let digit = unsafe { s.get_unchecked(i_start + i) } - b'0';
            digit_counts[i][digit as usize] += 1;
        }

        i_start += substring_len + 2;
    }

    let mut n_common_positions = 0;
    for counts in digit_counts {
        for c in counts {
            n_common_positions += c * (c - 1) / 2;
        }
    }

    n_common_positions as f64 / ((n_substrings * (n_substrings - 1)) * substring_len / 2) as f64
        * 100.
}
