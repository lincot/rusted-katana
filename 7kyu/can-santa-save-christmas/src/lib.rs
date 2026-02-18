//! <https://www.codewars.com/kata/5857e8bb9948644aa1000246/train/rust>

pub const fn determine_time(durations: &[&str]) -> bool {
    let mut secs = 0;
    let mut i = 0;
    while i < durations.len() {
        let duration = durations[i].as_bytes();
        assert!(duration.len() == 8);
        secs += 3600
            * (10 * (duration[0] as usize - b'0' as usize)
                + (duration[1] as usize - b'0' as usize))
            + 60 * (10 * (duration[3] as usize - b'0' as usize)
                + (duration[4] as usize - b'0' as usize))
            + (10 * (duration[6] as usize - b'0' as usize) + duration[7] as usize - b'0' as usize);
        if secs > 24 * 3600 {
            return false;
        }
        i += 1;
    }
    true
}
