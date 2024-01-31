use core::fmt::Write;
use see_you_next_happy_year::next_happy_year;

const fn is_happy(mut n: u16) -> bool {
    let d4 = n % 10;
    n /= 10;
    let d3 = n % 10;
    n /= 10;
    let d2 = n % 10;
    n /= 10;
    let d1 = n % 10;

    d1 != d2 && d1 != d3 && d1 != d4 && d2 != d3 && d2 != d4 && d3 != d4
}

fn next_happy_year_iterative(year: u16) -> u16 {
    (year + 1..).find(|&n| is_happy(n)).unwrap()
}

#[test]
fn test() {
    let mut panic_message = String::new();
    let mut failures_count = 0;

    for year in 1000..=if cfg!(miri) { 1100 } else { 9000 } {
        let expected = next_happy_year_iterative(year);
        let got = next_happy_year(year);

        if expected != got {
            writeln!(panic_message, "{year}: expected {expected}, got {got}").unwrap();
            failures_count += 1;
        }
    }

    assert!(
        failures_count == 0,
        "\n{panic_message}\na total of {failures_count} failures",
    );
}
