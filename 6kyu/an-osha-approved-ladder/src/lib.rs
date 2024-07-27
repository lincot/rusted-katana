//! <https://www.codewars.com/kata/65116501a5de2bc51f409c1a/train/rust>

use core::hint::unreachable_unchecked;

pub fn is_ladder_safe(ldr: &[&str]) -> bool {
    if ldr.len() < 3 {
        return false;
    }

    let width = ldr[0].len();
    if width < 5 {
        return false;
    }
    for row in &ldr[1..] {
        if row.len() != width {
            return false;
        }
    }

    if unsafe {
        get_row_type(ldr[0]) != RowType::Space
            || get_row_type(ldr[ldr.len() - 1]) != RowType::Space
            || get_row_type(ldr[1]) != RowType::Rung
            || get_row_type(ldr[ldr.len() - 2]) != RowType::Rung
    } {
        return false;
    }

    if ldr.len() == 3 {
        return true;
    }

    let Some(second_runk_pos) = ldr[2..]
        .iter()
        .position(|row| unsafe { get_row_type(row) } == RowType::Rung)
        .map(|x| x + 2)
    else {
        return false;
    };

    let space_between = second_runk_pos - 2;

    if space_between == 0 {
        #[allow(clippy::needless_range_loop)]
        for i in second_runk_pos + 1..ldr.len() - 2 {
            if unsafe { get_row_type(ldr[i]) != RowType::Rung } {
                return false;
            }
        }
    } else if space_between == 1 {
        if ldr.len() % 2 == 0 {
            return false;
        }

        let mut i = second_runk_pos + 1;
        while i < ldr.len() - 1 {
            if unsafe {
                get_row_type(ldr[i]) != RowType::Space || get_row_type(ldr[i + 1]) != RowType::Rung
            } {
                return false;
            }

            i += 2;
        }
    } else if space_between == 2 {
        if ldr.len() % 3 != 0 {
            return false;
        }

        let mut i = second_runk_pos + 1;
        while i < ldr.len() - 1 {
            if unsafe {
                get_row_type(ldr[i]) != RowType::Space
                    || get_row_type(ldr[i + 1]) != RowType::Space
                    || get_row_type(ldr.get_unchecked(i + 2)) != RowType::Rung
            } {
                return false;
            }

            i += 3;
        }
    } else {
        return false;
    }

    true
}

#[derive(PartialEq, Eq)]
enum RowType {
    Rung,
    Space,
    BrokenRung,
}

unsafe fn get_row_type(row: &str) -> RowType {
    let row = row.as_bytes();
    if row.len() < 5 {
        unreachable_unchecked();
    }

    if row[0] != b'#' || row[row.len() - 1] != b'#' {
        return RowType::BrokenRung;
    }

    if row[1..row.len() - 1].iter().all(|&b| b == b'#') {
        RowType::Rung
    } else if row[1..row.len() - 1].iter().all(|&b| b == b' ') {
        RowType::Space
    } else {
        RowType::BrokenRung
    }
}
