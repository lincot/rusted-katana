//! <https://www.codewars.com/kata/5bf71b94e50d1b00590000fe/train/rust>

use core::hint::unreachable_unchecked;

pub fn count_squares(lines: &[&str]) -> usize {
    let mut res = 0;
    for i in 0..lines.len() {
        let up_line = lines[i].as_bytes();
        for j in 0..up_line.len() {
            if up_line[j] == b'+' {
                #[allow(clippy::needless_range_loop)]
                for j_ in j + 1..up_line.len().min(lines.len() - i + j) {
                    match up_line[j_] {
                        b'+' => {
                            let down_line = unsafe { lines.get_unchecked(i + j_ - j) }.as_bytes();
                            if let Some(slice) = down_line.get(j..=j_) {
                                if slice.len() < 2 {
                                    unsafe { unreachable_unchecked() };
                                }
                                if slice[0] == b'+'
                                    && slice[slice.len() - 1] == b'+'
                                    && slice[1..slice.len() - 1].iter().all(|b| !b" |".contains(b))
                                    && (i + 1..i + j_ - j).all(|ind| {
                                        let between_line =
                                            unsafe { lines.get_unchecked(ind) }.as_bytes();
                                        for column in [j, j_] {
                                            if [Some(&b' '), Some(&b'-'), None]
                                                .contains(&between_line.get(column))
                                            {
                                                return false;
                                            }
                                        }
                                        true
                                    })
                                {
                                    res += 1;
                                }
                            }
                        }
                        b' ' | b'|' => break,
                        _ => {}
                    }
                }
            }
        }
    }
    res
}
