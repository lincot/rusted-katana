//! <https://www.codewars.com/kata/6375587af84854823ccd0e90/train/rust>

use unchecked_std::prelude::*;

pub fn block_print(input: &str) -> String {
    let input = input.as_bytes();
    let mut start = 0;
    loop {
        if start == input.len() {
            return String::new();
        }
        if input[start] != b' ' {
            break;
        }
        start += 1;
    }
    let mut end = input.len();
    loop {
        if *unsafe { input.get_unchecked(end - 1) } != b' ' {
            break;
        }
        end -= 1;
    }
    let input = unsafe { input.get_unchecked(start..end) };
    let mut res = String::with_capacity(input.len().checked_mul(6 * 7).unwrap());

    unsafe {
        for row in 0..7 {
            push_row(&mut res, input, row);
        }
    }
    res
}

unsafe fn push_row(res: &mut String, input: &[u8], row: usize) {
    unsafe {
        for &b in input {
            if b.is_ascii_lowercase() {
                push_line(res, row, b - b'a');
            } else if b.is_ascii_uppercase() {
                push_line(res, row, b - b'A');
            } else {
                res.push_str_unchecked("      ");
            }
        }
        while *res.as_bytes().last().unwrap_unchecked() == b' ' {
            res.pop();
        }
        if row != 6 {
            res.push_unchecked('\n');
        }
    }
}

unsafe fn push_line(res: &mut String, row: usize, b: u8) {
    let i = 156 * row + 6 * b as usize;
    unsafe { res.push_str_unchecked(ALPHABET.get_unchecked(i..i + 6)) };
}

const ALPHABET: &str = concat!(
    " AAA  BBBB   CCC  DDDD  EEEEE FFFFF  GGG  H   H IIIII JJJJJ K   K L     M   M N   N  OOO  PPPP   QQQ  RRRR   SSS  TTTTT U   U V   V W   W X   X Y   Y ZZZZZ ",
    "A   A B   B C   C D   D E     F     G   G H   H   I       J K  K  L     MM MM NN  N O   O P   P Q   Q R   R S   S   T   U   U V   V W   W X   X Y   Y     Z ",
    "A   A B   B C     D   D E     F     G     H   H   I       J K K   L     M M M N   N O   O P   P Q   Q R   R S       T   U   U V   V W   W  X X   Y Y     Z  ",
    "AAAAA BBBB  C     D   D EEEEE FFFFF G GGG HHHHH   I       J KK    L     M   M N N N O   O PPPP  Q   Q RRRR   SSS    T   U   U V   V W W W   X     Y     Z   ",
    "A   A B   B C     D   D E     F     G   G H   H   I       J K K   L     M   M N   N O   O P     Q Q Q R R       S   T   U   U V   V W W W  X X    Y    Z    ",
    "A   A B   B C   C D   D E     F     G   G H   H   I       J K  K  L     M   M N  NN O   O P     Q  QQ R  R  S   S   T   U   U  V V  W W W X   X   Y   Z     ",
    "A   A BBBB   CCC  DDDD  EEEEE F      GGG  H   H IIIII JJJJ  K   K LLLLL M   M N   N  OOO  P      QQQQ R   R  SSS    T    UUU    V    W W  X   X   Y   ZZZZZ "
);
