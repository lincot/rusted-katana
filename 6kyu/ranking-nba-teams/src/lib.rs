//! <https://www.codewars.com/kata/5a420163b6cfd7cde5000077/train/rust>

use core::cmp::Ordering;
use digital::{MaxLenBase10, WriteNumUnchecked};
use unchecked_std::prelude::*;

pub fn nba_cup(ro: &str, to_find: &str) -> String {
    const ERROR_FLOAT: &str = "Error(float number):";

    if to_find.is_empty() {
        return String::new();
    }
    let [mut wins, mut draws, mut loses, mut scored, mut conceded] = [0usize; 5];
    for (i, _) in ro.match_indices(to_find) {
        if ro.as_bytes()[i + to_find.len()] != b' ' {
            continue;
        }
        let (score, opponent_score) = if i == 0 || ro.as_bytes()[i - 1] == b',' {
            let score_index = i + to_find.len() + 1;
            let space_index = score_index
                + ro.as_bytes()[score_index..]
                    .iter()
                    .position(|&b| b == b' ')
                    .unwrap();
            let part_end = unsafe { ro.as_bytes().get_unchecked(space_index..) }
                .iter()
                .position(|&b| b == b',')
                .map_or(ro.len(), |x| x + space_index);
            let error_float = || {
                let mut res = String::with_capacity(ERROR_FLOAT.len() + part_end - i);
                unsafe {
                    res.push_str_unchecked(ERROR_FLOAT);
                    res.push_str_unchecked(ro.get_unchecked(i..part_end));
                }
                res
            };
            let score_str = unsafe { ro.get_unchecked(score_index..space_index) };
            if score_str.as_bytes().contains(&b'.') {
                return error_float();
            }
            let score: u8 = score_str.parse().unwrap();
            let opponent_score_index = part_end
                - unsafe { ro.as_bytes().get_unchecked(..part_end) }
                    .iter()
                    .rev()
                    .position(|&b| b == b' ')
                    .unwrap();
            let opponent_score_str = unsafe { ro.get_unchecked(opponent_score_index..part_end) };
            if opponent_score_str.as_bytes().contains(&b'.') {
                return error_float();
            }
            let opponent_score: u8 = opponent_score_str.parse().unwrap();
            (score, opponent_score)
        } else {
            if ro.as_bytes()[i - 1] != b' ' {
                continue;
            }

            let score_index = i + to_find.len() + 1;
            let part_end = ro.as_bytes()[score_index..]
                .iter()
                .position(|&b| b == b',')
                .map_or(ro.len(), |x| x + score_index);
            let error_float = |p| {
                let start_index = unsafe { ro.as_bytes().get_unchecked(..p) }
                    .iter()
                    .rev()
                    .position(|&b| b == b',')
                    .map_or(0, |x| p - x);
                let mut res = String::with_capacity(ERROR_FLOAT.len() + part_end - start_index);
                unsafe {
                    res.push_str_unchecked(ERROR_FLOAT);
                    res.push_str_unchecked(ro.get_unchecked(start_index..part_end));
                }
                res
            };
            let score_str = unsafe { ro.get_unchecked(score_index..part_end) };
            if score_str.as_bytes().contains(&b'.') {
                return error_float(i);
            }
            let score: u8 = score_str.parse().unwrap();
            let opponent_score_index = i
                - 1
                - ro.as_bytes()[..i - 1]
                    .iter()
                    .rev()
                    .position(|&b| b == b' ')
                    .unwrap();
            let opponent_score_str = unsafe { ro.get_unchecked(opponent_score_index..i - 1) };
            if opponent_score_str.as_bytes().contains(&b'.') {
                return error_float(opponent_score_index);
            }
            let opponent_score: u8 = opponent_score_str.parse().unwrap();
            (score, opponent_score)
        };

        match score.cmp(&opponent_score) {
            Ordering::Less => loses += 1,
            Ordering::Greater => wins += 1,
            Ordering::Equal => draws += 1,
        }
        scored += score as usize;
        conceded += opponent_score as usize;
    }
    let mut res = String::with_capacity(
        to_find.len() + ":W=;D=;L=;Scored=;Conceded=;Points=".len() + 6 * usize::MAX_LEN_BASE10,
    );
    unsafe {
        res.push_str_unchecked(to_find);
        if wins == 0 && loses == 0 && draws == 0 {
            res.push_str_unchecked(":This team didn't play!");
            return res;
        }
        res.push_str_unchecked(":W=");
        res.write_num_unchecked(wins, 10, false, false);
        res.push_str_unchecked(";D=");
        res.write_num_unchecked(draws, 10, false, false);
        res.push_str_unchecked(";L=");
        res.write_num_unchecked(loses, 10, false, false);
        res.push_str_unchecked(";Scored=");
        res.write_num_unchecked(scored, 10, false, false);
        res.push_str_unchecked(";Conceded=");
        res.write_num_unchecked(conceded, 10, false, false);
        res.push_str_unchecked(";Points=");
        res.write_num_unchecked(3 * wins + draws, 10, false, false);
    }
    res
}
