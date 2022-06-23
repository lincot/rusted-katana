//! <https://www.codewars.com/kata/5b76a34ff71e5de9db0000f2/train/rust>

fn parse_time(time: &str) -> u32 {
    assert_eq!(time.len(), 5);

    let hours: u8 = time[..2].parse().unwrap();
    let minutes: u8 = time[3..].parse().unwrap();

    60 * hours as u32 + minutes as u32
}

fn format_time(time: u32) -> String {
    format!("{:02}:{:02}", time / 60, time % 60)
}

pub fn solve(arr: &[&str]) -> String {
    assert!(!arr.is_empty());

    let mut arr: Vec<_> = arr.iter().map(|s| parse_time(s)).collect();
    arr.sort_unstable();

    let max_diff = arr
        .windows(2)
        .map(|pair| pair[1] - pair[0])
        .max()
        .unwrap_or_default()
        .max(24 * 60 + arr[0] - arr[arr.len() - 1]);

    format_time(max_diff - 1)
}
