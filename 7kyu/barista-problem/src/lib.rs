//! <https://www.codewars.com/kata/6167e70fc9bd9b00565ffa4e/train/rust>

pub fn barista(coffees: &[u8]) -> u16 {
    let mut coffees: Box<[_]> = coffees.into();
    if coffees.len() < 32 {
        coffees.sort_unstable();
    } else {
        radsort::sort(&mut coffees);
    }
    let mut sum_of_prev = 0;
    let mut res = coffees.len() as u16 * coffees.len() as u16 - coffees.len() as u16;
    for &coffee in &*coffees {
        sum_of_prev += coffee as u16;
        res += sum_of_prev;
    }
    res
}
