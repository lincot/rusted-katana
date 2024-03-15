//! <https://www.codewars.com/kata/5b483e70a4bc16eda40000f9/train/rust>

pub fn longest_bouncy_list(arr: &[i32]) -> Vec<i32> {
    if arr.is_empty() {
        return vec![];
    }
    if arr.len() == 1 {
        return arr.into();
    }

    let mut direction = 0;
    let mut max_len = 1;
    let mut end = 0;
    let mut len = 1;
    for i in 1..arr.len() {
        let new_direction = arr[i].cmp(&arr[i - 1]) as i8;
        if direction == 0 {
            if new_direction == 0 {
                len = 1;
            } else {
                len = 2;
            }
        } else if new_direction == -direction {
            len += 1;
            // we could simplify the ifs but it makes it slower...
        } else if new_direction == 0 {
            len = 1;
        } else {
            len = 2;
        }

        direction = new_direction;
        if len > max_len {
            max_len = len;
            end = i;
        }
    }

    unsafe { arr.get_unchecked(end + 1 - max_len..=end) }.into()
}
