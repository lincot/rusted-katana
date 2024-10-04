//! <https://www.codewars.com/kata/55b3425df71c1201a800009c/train/rust>

use unchecked_std::prelude::*;

pub fn stati(strg: &str) -> String {
    if strg.is_empty() {
        return String::new();
    }

    let mut results = Vec::with_capacity(strg.len() / "1|1|1".len());
    for mut runner in strg.as_bytes().split(|&b| b == b',') {
        if runner.starts_with(b" ") {
            runner = &runner[1..];
        }
        assert!(runner.len() >= "1|1|1".len());

        let (hours, minutes_pos) = if runner[1].is_ascii_digit() {
            (
                10 * (runner[0] as u32 - b'0' as u32) + runner[1] as u32 - b'0' as u32,
                3,
            )
        } else {
            (runner[0] as u32 - b'0' as u32, 2)
        };

        let (minutes, seconds_pos) = if runner[minutes_pos + 1].is_ascii_digit() {
            (
                10 * (runner[minutes_pos] as u32 - b'0' as u32) + runner[minutes_pos + 1] as u32
                    - b'0' as u32,
                minutes_pos + 3,
            )
        } else {
            (runner[minutes_pos] as u32 - b'0' as u32, minutes_pos + 2)
        };

        let seconds = if seconds_pos + 1 < runner.len() && runner[seconds_pos + 1].is_ascii_digit()
        {
            10 * (runner[seconds_pos] as u32 - b'0' as u32) + runner[seconds_pos + 1] as u32
                - b'0' as u32
        } else {
            runner[seconds_pos] as u32 - b'0' as u32
        };

        unsafe { results.push_unchecked(3600 * hours + 60 * minutes + seconds) };
    }

    // there are not many players in a team, so the insertion sort works fine
    results.sort_unstable();
    let range = results.last().unwrap() - results.first().unwrap();
    let average = results.iter().sum::<u32>() / results.len() as u32;
    let median = if results.len() % 2 == 0 {
        (results[results.len() / 2 - 1] + results[results.len() / 2]) / 2
    } else {
        results[results.len() / 2]
    };

    let mut res = String::with_capacity("Range: 01|01|18 Average: 01|38|05 Median: 01|32|34".len());
    unsafe {
        res.push_str_unchecked("Range: ");
        format_time_unchecked(&mut res, range);
        res.push_str_unchecked(" Average: ");
        format_time_unchecked(&mut res, average);
        res.push_str_unchecked(" Median: ");
        format_time_unchecked(&mut res, median);
    }
    res
}

unsafe fn format_time_unchecked(s: &mut String, total_seconds: u32) {
    let s = s.as_mut_vec();

    let hours = total_seconds / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    s.push_unchecked((hours / 10) as u8 + b'0');
    s.push_unchecked((hours % 10) as u8 + b'0');
    s.push_unchecked(b'|');
    s.push_unchecked((minutes / 10) as u8 + b'0');
    s.push_unchecked((minutes % 10) as u8 + b'0');
    s.push_unchecked(b'|');
    s.push_unchecked((seconds / 10) as u8 + b'0');
    s.push_unchecked((seconds % 10) as u8 + b'0');
}
