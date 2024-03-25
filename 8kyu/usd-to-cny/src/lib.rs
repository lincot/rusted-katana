//! <https://www.codewars.com/kata/5977618080ef220766000022/train/rust>

use digital::WriteNumUnchecked;
use unchecked::{PushStrUnchecked, PushUnchecked};

pub fn usdcny(usd: u16) -> String {
    let mut res = String::with_capacity("442361.25 Chinese Yuan".len());
    unsafe {
        res.write_num_unchecked(usd as u32 * 675, 10, false, false);
        for _ in 0..3usize.saturating_sub(res.len()) {
            res.push_unchecked('0');
        }
        let len_before = res.len();
        let last = *res.as_bytes().get_unchecked(len_before - 1);
        let penult = *res.as_bytes().get_unchecked(len_before - 2);
        res.as_mut_vec().push_unchecked(last);
        *res.as_mut_vec().get_unchecked_mut(len_before - 1) = penult;
        *res.as_mut_vec().get_unchecked_mut(len_before - 2) = b'.';
        res.push_str_unchecked(" Chinese Yuan");
    }
    res
}
