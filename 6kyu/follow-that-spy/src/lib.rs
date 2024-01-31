//! <https://www.codewars.com/kata/5899a4b1a6648906fe000113/train/rust>

use core::fmt::{Debug, Display};
use unchecked::PushStrUnchecked;

pub fn find_routes<S>(routes: &[[S; 2]]) -> String
where
    S: AsRef<str> + Debug + Display,
{
    let first = routes
        .iter()
        .position(|[a, _]| !routes.iter().any(|[_, b]| a.as_ref() == b.as_ref()))
        .unwrap();
    let mut cap = routes[first][0].as_ref().len() + 2 * routes.len();
    for [_, b] in routes {
        cap += b.as_ref().len();
    }
    let mut res = String::with_capacity(cap);
    let mut used = vec![false; routes.len()];
    unsafe {
        res.push_str_unchecked(routes[first][0].as_ref());
        res.push_str_unchecked(", ");
        res.push_str_unchecked(routes[first][1].as_ref());
    }
    used[first] = true;
    let mut prev = &routes[first][1];
    loop {
        let Some((i, [_, b])) = routes
            .iter()
            .enumerate()
            .find(|&(i, [a, _])| !*unsafe { used.get_unchecked(i) } && a.as_ref() == prev.as_ref())
        else {
            break;
        };
        unsafe {
            *used.get_unchecked_mut(i) = true;
            res.push_str_unchecked(", ");
            res.push_str_unchecked(b.as_ref());
        }
        prev = b;
    }
    res
}
