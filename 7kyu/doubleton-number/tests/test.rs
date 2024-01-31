use core::fmt::Write;
use doubleton_number::doubleton;

const fn is_doubleton(mut n: u32) -> bool {
    let mut d1 = 10;
    let mut d2 = 10;

    while n != 0 {
        let d = n % 10;

        if d1 != d && d2 != d {
            if d2 != 10 {
                return false;
            }
            d2 = d1;
            d1 = d;
        }

        n /= 10;
    }

    d2 != 10
}

pub fn doubleton_iterative(num: u32) -> u32 {
    (num + 1..).find(|&x| is_doubleton(x)).unwrap()
}

#[test]
fn test() {
    let mut panic_message = String::new();
    let mut failures_count = 0;

    for num in 1..=if cfg!(miri) { 100 } else { 1_000_000 } {
        let expected = doubleton_iterative(num);
        let got = doubleton(num);

        if expected != got {
            writeln!(panic_message, "{num}: expected {expected}, got {got}").unwrap();
            failures_count += 1;
        }
    }

    assert!(
        failures_count == 0,
        "\n{panic_message}\na total of {failures_count} failures",
    );
}
