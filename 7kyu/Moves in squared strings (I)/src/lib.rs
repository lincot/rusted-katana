//! <https://www.codewars.com/kata/56dbe0e313c2f63be4000b25/train/rust>

pub fn hor_mirror(s: String) -> String {
    let mut rev_lines = s.rsplit('\n');

    let first = if let Some(first) = rev_lines.next() {
        first
    } else {
        return s;
    };

    let mut res = String::with_capacity(s.len());

    res.push_str(first);

    for line in rev_lines {
        res.push('\n');
        res.push_str(line);
    }

    res
}

pub fn vert_mirror(s: String) -> String {
    let mut lines = s.split('\n');

    let first = if let Some(first) = lines.next() {
        first
    } else {
        return s;
    };

    let mut res = String::with_capacity(s.len());

    res.extend(first.chars().rev());

    for line in lines {
        res.push('\n');
        res.extend(line.chars().rev());
    }

    res
}

pub fn oper(f: fn(String) -> String, s: String) -> String {
    f(s)
}
