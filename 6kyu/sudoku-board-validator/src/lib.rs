//! <https://www.codewars.com/kata/63d1bac72de941033dbf87ae/train/rust>

pub fn validate_sudoku(sudoku: &[[u8; 9]; 9]) -> bool {
    const TARGET: u16 = (1 << 10) - 2;

    for row in sudoku {
        let mut bitset = 0u16;
        for x in row {
            bitset ^= 1 << x;
        }
        if bitset != TARGET {
            return false;
        }
    }

    for j in 0..9 {
        let mut bitset = 0u16;
        for row in sudoku {
            bitset ^= 1 << row[j];
        }
        if bitset != TARGET {
            return false;
        }
    }

    for i in [0, 3, 6] {
        for j in [0, 3, 6] {
            let mut bitset = 0u16;
            for i_offset in 0..3 {
                for j_offset in 0..3 {
                    bitset ^= 1
                        << unsafe {
                            sudoku
                                .get_unchecked(i + i_offset)
                                .get_unchecked(j + j_offset)
                        };
                }
            }
            if bitset != TARGET {
                return false;
            }
        }
    }

    true
}
