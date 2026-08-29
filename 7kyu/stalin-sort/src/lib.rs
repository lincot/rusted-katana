//! <https://www.codewars.com/kata/699af631058f5c12b04f4efe/train/rust>

pub fn stalin_sort(vector: &mut Vec<i32>) {
    let Some(mut max) = vector.first().copied() else {
        return;
    };

    let mut i = 0;
    let mut j = 1;
    while j < vector.len() {
        let x = vector[j];
        if x >= max {
            i += 1;
            unsafe { *vector.get_unchecked_mut(i) = x };
            max = x;
        }
        j += 1;
    }
    unsafe { vector.set_len(i + 1) };
}
