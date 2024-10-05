//! <https://www.codewars.com/kata/5f6d120d40b1c900327b7e22/train/rust>

pub fn leaderboard_sort(leaderboard: &[String], changes: &[String]) -> Vec<String> {
    let mut leaderboard = leaderboard.to_vec();
    for change in changes {
        let (name, change) = change.split_once(' ').unwrap();
        let change: isize = change.parse().unwrap();

        let pos = leaderboard.iter().position(|s| s == name).unwrap();
        if change > 0 {
            leaderboard[(pos as isize - change) as _..pos + 1].rotate_right(1);
        } else {
            leaderboard[pos..(pos as isize - change + 1) as _].rotate_left(1);
        }
    }
    leaderboard
}
