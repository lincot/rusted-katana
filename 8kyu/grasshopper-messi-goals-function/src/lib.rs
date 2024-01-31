//! <https://www.codewars.com/kata/55f73be6e12baaa5900000d4/train/rust>

pub const fn goals(
    la_liga_goals: i32,
    champions_league_goals: i32,
    copa_del_rey_goals: i32,
) -> i32 {
    la_liga_goals + champions_league_goals + copa_del_rey_goals
}
