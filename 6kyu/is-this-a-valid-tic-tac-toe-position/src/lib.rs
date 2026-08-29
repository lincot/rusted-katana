//! <https://www.codewars.com/kata/68582183cf16719a5ba943df/train/rust>

mod preloaded;

pub use crate::preloaded::Player::{self, *};

pub fn is_valid_position(board: &[[Player; 3]; 3]) -> bool {
    let x_cnt: usize = board
        .iter()
        .map(|row| row.iter().filter(|&&x| x == X).count())
        .sum();
    let o_cnt: usize = board
        .iter()
        .map(|row| row.iter().filter(|&&x| x == O).count())
        .sum();
    if x_cnt != o_cnt && x_cnt != o_cnt + 1 {
        return false;
    }

    let mut x_triples_cnt = 0;
    let mut o_triples_cnt = 0;

    for row in board {
        if row == &[X; 3] {
            x_triples_cnt += 1;
        }
        if row == &[O; 3] {
            o_triples_cnt += 1;
        }
    }

    for j in 0..3 {
        if (0..3).all(|i| board[i][j] == X) {
            x_triples_cnt += 1;
        }
        if (0..3).all(|i| board[i][j] == O) {
            o_triples_cnt += 1;
        }
    }

    if (0..3).all(|i| board[i][i] == X) {
        x_triples_cnt += 1;
    }
    if (0..3).all(|i| board[i][i] == O) {
        o_triples_cnt += 1;
    }
    if (0..3).all(|i| board[i][2 - i] == X) {
        x_triples_cnt += 1;
    }
    if (0..3).all(|i| board[i][2 - i] == O) {
        o_triples_cnt += 1;
    }

    #[expect(clippy::nonminimal_bool)]
    !(x_triples_cnt > 0 && o_triples_cnt > 0
        || x_triples_cnt > 0 && x_cnt == o_cnt
        || o_triples_cnt > 0 && x_cnt > o_cnt)
}
