use find_the_missing_term_in_an_arithmetic_progression::find_missing;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;

#[test]
fn random_tests() {
    let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);
    for _ in 0..1000 {
        let r = rng.random_range(-100..=100);
        let l = rng.random_range(4..=100);
        let start = rng.random_range(-1_000_000..=1_000_000);
        let mut seq = (0..l).map(|e| start + e as i32 * r).collect::<Vec<_>>();
        let j = rng.random_range(1..l - 1);
        let expected = seq.remove(j);
        assert_eq!(find_missing(&seq), expected);
    }
}
