//! <https://www.codewars.com/kata/56f6ad906b88de513f000d96/train/rust>

pub fn bonus_time(mut salary: u64, bonus: bool) -> String {
    if bonus {
        salary *= 10;
    }

    format!("¥{salary}")
}
