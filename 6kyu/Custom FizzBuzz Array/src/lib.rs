//! <https://www.codewars.com/kata/5355a811a93a501adf000ab7/train/rust>

pub fn fizz_buzz_custom_solver(
    string_one: &str,
    string_two: &str,
    num_one: usize,
    num_two: usize,
) -> Vec<String> {
    (1..=100)
        .map(|x| match (x % num_one, x % num_two) {
            (0, 0) => {
                let mut res = String::with_capacity(string_one.len() + string_two.len());
                res.push_str(string_one);
                res.push_str(string_two);
                res
            }
            (0, _) => string_one.into(),
            (_, 0) => string_two.into(),
            _ => x.to_string(),
        })
        .collect()
}

#[macro_export]
macro_rules! fizz_buzz_custom {
    () => {
        fizz_buzz_custom_solver("Fizz", "Buzz", 3, 5)
    };
    ($str_one:expr) => {
        fizz_buzz_custom_solver($str_one, "Buzz", 3, 5)
    };
    ($str_one:expr, $str_two:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, 3, 5)
    };
    ($str_one:expr, $str_two:expr, $num_one:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, $num_one, 5)
    };
    ($str_one:expr, $str_two:expr, $num_one:expr, $num_two:expr) => {
        fizz_buzz_custom_solver($str_one, $str_two, $num_one, $num_two)
    };
}
