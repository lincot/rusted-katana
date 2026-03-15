//! <https://www.codewars.com/kata/58b3c2bd917a5caec0000017/train/rust>

pub fn sum_groups(arr: &[u32]) -> usize {
    let mut even = arr[0].is_multiple_of(2);
    let mut count = 1;

    let mut prev_value = None;
    let mut res = 0;

    for x in &arr[1..] {
        if x.is_multiple_of(2) == even {
            count += 1;
        } else {
            let curr_value = even || count % 2 == 0;
            if prev_value != Some(curr_value) {
                prev_value = Some(curr_value);
                res += 1;
            }

            even = !even;
            count = 1;
        }
    }

    let curr_value = even || count % 2 == 0;
    if prev_value != Some(curr_value) {
        res += 1;
    }

    res
}
