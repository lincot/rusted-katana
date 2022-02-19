//! <https://www.codewars.com/kata/56f3a1e899b386da78000732/train/rust>

pub fn part_list(arr: Vec<&str>) -> String {
    match arr.len() {
        0 => return String::new(),
        1 => return format!("({})", arr[0]),
        _ => {}
    }

    // arbitrary capacity
    let cap = 5 * arr.len();
    let mut joined = String::with_capacity(cap);
    let mut comma_poses = Vec::with_capacity(arr.len() - 1);

    for s in arr[..arr.len() - 1].iter() {
        joined.push_str(s);
        comma_poses.push(joined.len());
        joined.push(' ');
    }

    joined.push_str(arr[arr.len() - 1]);

    let mut res = String::with_capacity(comma_poses.len() * (joined.len() + 3));

    for comma_pos in comma_poses {
        res.push('(');
        res.push_str(unsafe { joined.get_unchecked(..comma_pos) });
        res.push(',');
        res.push_str(unsafe { joined.get_unchecked(comma_pos..) });
        res.push(')');
    }

    res
}
