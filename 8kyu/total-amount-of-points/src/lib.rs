//! <https://www.codewars.com/kata/5bb904724c47249b10000131/train/rust>

use std::cmp::Ordering;

pub fn points(games: &[String]) -> u32 {
    let mut res = 0;
    for game in games {
        assert!(game.len() >= 3);
        let (x, y) = (game.as_bytes()[0], game.as_bytes()[2]);
        res += match x.cmp(&y) {
            Ordering::Greater => 3,
            Ordering::Less => 0,
            Ordering::Equal => 1,
        };
    }
    res
}
