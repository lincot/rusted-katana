//! <https://www.codewars.com/kata/55d8aa568dec9fb9e200004a/train/rust>

pub fn sel_number(n: u32, d: u8) -> u32 {
    let d = d as u32;
    let mut res = 0;
    for start_digit in 1..10 {
        res += sel_number_recursive(start_digit, n, d, start_digit);
    }
    res
}

fn sel_number_recursive(last_digit: u32, max_num: u32, max_diff: u32, current: u32) -> u32 {
    let mut count = 0;
    for next_digit in (last_digit + 1)..(last_digit + 1 + max_diff).min(10) {
        let new_num = current * 10 + next_digit;
        if new_num > max_num {
            return count;
        }
        count += 1;
        count += sel_number_recursive(next_digit, max_num, max_diff, new_num);
    }
    count
}
