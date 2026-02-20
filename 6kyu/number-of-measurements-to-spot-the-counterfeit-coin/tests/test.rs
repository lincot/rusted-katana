use number_of_measurements_to_spot_the_counterfeit_coin::how_many_measurements;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;

fn naive_solution(n: u64) -> u32 {
    match n {
        1 => 0,
        2 | 3 => 1,
        _ => {
            if n.is_multiple_of(3) {
                1 + naive_solution(n / 3)
            } else {
                1 + naive_solution((n / 3) + 1)
            }
        }
    }
}

#[test]
fn random_tests() {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    for _ in 0..40 {
        let n = rng.random();
        assert_eq!(how_many_measurements(n), naive_solution(n));
    }
}

#[test]
fn big_tests() {
    assert_eq!(how_many_measurements(u64::MAX), naive_solution(u64::MAX));
    assert_eq!(
        how_many_measurements(3u64.pow(35) - 5),
        naive_solution(3u64.pow(35) - 5)
    );
    assert_eq!(
        how_many_measurements(3u64.pow(40) - 5),
        naive_solution(3u64.pow(40) - 5)
    );
    for exp in 0..=40 {
        assert_eq!(
            how_many_measurements(3u64.pow(exp)),
            naive_solution(3u64.pow(exp))
        );
    }
}
