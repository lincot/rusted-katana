//! <https://www.codewars.com/kata/58b3c2bd917a5caec0000017/train/rust>

#[derive(PartialEq, Eq)]
enum MaybeBool {
    False,
    True,
    None,
}

impl From<bool> for MaybeBool {
    fn from(value: bool) -> Self {
        if value { Self::True } else { Self::False }
    }
}

pub fn sum_groups(arr: &[u32]) -> usize {
    let mut even = arr[0].is_multiple_of(2);
    let mut count = 1;

    let mut prev_value = MaybeBool::None;
    let mut res = 0;

    for x in &arr[1..] {
        if x.is_multiple_of(2) == even {
            count += 1;
        } else {
            let curr_value = MaybeBool::from(even || count % 2 == 0);
            res += (curr_value != prev_value) as usize;
            prev_value = curr_value;

            even = !even;
            count = 1;
        }
    }

    let curr_value = MaybeBool::from(even || count % 2 == 0);
    res += (curr_value != prev_value) as usize;

    res
}
