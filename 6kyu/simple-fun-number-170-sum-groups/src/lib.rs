//! <https://www.codewars.com/kata/58b3c2bd917a5caec0000017/train/rust>

pub fn sum_groups(arr: &[u32]) -> usize {
    let mut arr = arr.to_vec();

    loop {
        let mut odd = arr[0] % 2 == 1;
        let mut i = 0;
        let mut new_len = 0;
        while i < arr.len() {
            let mut sum = 0;
            while i < arr.len() && arr[i] % 2 == odd as u32 {
                sum += arr[i];
                i += 1;
            }

            odd = !odd;
            unsafe { *arr.get_unchecked_mut(new_len) = sum };
            new_len += 1;
        }

        if new_len >= arr.len() {
            return arr.len();
        }
        arr.truncate(new_len);
    }
}
