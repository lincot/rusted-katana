//! <https://www.codewars.com/kata/6167e70fc9bd9b00565ffa4e/train/rust>

pub fn barista(coffees: &[u8]) -> u16 {
    let mut coffees = coffees.to_vec();
    coffees.sort_unstable();
    let mut coffees = coffees.into_iter();
    let mut prev = if let Some(first) = coffees.next() {
        first as u16
    } else {
        return 0;
    };
    let mut res = prev;
    for coffee in coffees {
        prev += (coffee + 2) as u16;
        res += prev;
    }
    res
}
