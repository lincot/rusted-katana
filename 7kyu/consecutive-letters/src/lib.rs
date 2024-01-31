//! <https://www.codewars.com/kata/5ce6728c939bf80029988b57/train/rust>

pub fn solve(s: &str) -> bool {
    if s.len() > (b'z' - b'a' + 1) as usize {
        return false;
    }

    let mut counts = [0; (b'z' - b'a' + 1) as usize];
    for &b in s.as_bytes() {
        if b.is_ascii_lowercase() {
            counts[(b - b'a') as usize] += 1;
        } else {
            return false;
        }
    }

    for (i, count) in counts.into_iter().enumerate().skip_while(|&(_, b)| b == 0) {
        if count == 0 {
            return counts
                .get(i + 1..)
                .is_some_and(|s| s.iter().all(|&x| x == 0));
        } else if count != 1 {
            return false;
        }
    }

    true
}
