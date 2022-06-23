//! <https://www.codewars.com/kata/52761ee4cffbc69732000738/train/rust>

use std::cmp::Ordering;

pub fn good_vs_evil(good: &str, evil: &str) -> String {
    fn sum_worth(races: &str, worths: &[u32]) -> u32 {
        races
            .split_ascii_whitespace()
            .map(|count| count.parse::<u32>().unwrap())
            .zip(worths)
            .map(|(count, worth)| worth * count)
            .sum()
    }

    let good_worth = sum_worth(good, &[1, 2, 3, 3, 4, 10]);
    let evil_worth = sum_worth(evil, &[1, 2, 2, 2, 3, 5, 10]);

    match good_worth.cmp(&evil_worth) {
        Ordering::Greater => "Battle Result: Good triumphs over Evil",
        Ordering::Less => "Battle Result: Evil eradicates all trace of Good",
        Ordering::Equal => "Battle Result: No victor on this battle field",
    }
    .into()
}
