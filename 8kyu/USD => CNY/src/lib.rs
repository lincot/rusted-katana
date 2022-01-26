//! <https://www.codewars.com/kata/5977618080ef220766000022/train/rust>

pub fn usdcny(usd: u16) -> String {
    format!("{:.2} Chinese Yuan", usd as f64 * 6.75)
}
