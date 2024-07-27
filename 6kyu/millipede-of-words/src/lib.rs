//! <https://www.codewars.com/kata/6344701cd748a12b99c0dbc4/train/rust>

use unchecked_std::prelude::*;

pub fn millipede(words: &[&str]) -> bool {
    let mut counts = Vec::with_capacity(2 * words.len());
    for (i, word) in words.iter().enumerate() {
        let first_char = word.chars().next().unwrap();
        let last_char = word.chars().next_back().unwrap();

        if first_char == last_char {
            if !counts.iter().any(|&(ch, _, _)| ch == first_char) {
                unsafe { counts.push_unchecked((first_char, isize::MIN, None)) };
            }
        } else {
            if let Some((_, count, only_word)) =
                counts.iter_mut().find(|&&mut (ch, _, _)| ch == first_char)
            {
                if *count == isize::MIN {
                    *count = 1;
                } else {
                    *count += 1;
                }
                *only_word = None;
            } else {
                unsafe { counts.push_unchecked((first_char, 1, Some(i))) };
            }

            if let Some((_, count, only_word)) =
                counts.iter_mut().find(|&&mut (ch, _, _)| ch == last_char)
            {
                if *count == isize::MIN {
                    *count = -1;
                } else {
                    *count -= 1;
                }
                *only_word = None;
            } else {
                unsafe { counts.push_unchecked((last_char, -1, Some(i))) };
            }
        }
    }

    if counts.len() == 1 {
        return true;
    }

    let mut one_only_word = None;
    let mut minus_one_only_word = None;

    for (_, count, only_word) in counts {
        if count == 1 {
            if one_only_word.is_some() {
                return false;
            }
            one_only_word = Some(only_word);
        } else if count == -1 {
            if minus_one_only_word.is_some() {
                return false;
            }
            minus_one_only_word = Some(only_word);
        } else if count != 0 {
            return false;
        }
    }

    match (one_only_word, minus_one_only_word) {
        (Some(Some(e1)), Some(Some(e2))) if e1 == e2 => false,
        (Some(_), Some(_)) | (None, None) => true,
        _ => false,
    }
}
