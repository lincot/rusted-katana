//! <https://www.codewars.com/kata/57d5e850bfcdc545870000b7/train/rust>

pub fn dead_ant_count(ants: &str) -> u32 {
    let ants = ants.as_bytes();
    let (mut a, mut n, mut t, mut ant) = (0, 0, 0, 0);
    for (i, &b) in ants.iter().enumerate() {
        match b {
            b'a' => {
                a += 1;
                if ants.get(i + 1) == Some(&b'n') && ants.get(i + 2) == Some(&b't') {
                    ant += 1;
                }
            }
            b'n' => n += 1,
            b't' => t += 1,
            _ => {}
        }
    }
    a.max(n).max(t) - ant
}
