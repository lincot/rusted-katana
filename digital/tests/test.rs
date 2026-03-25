use digital::NumToString;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;

macro_rules! gen_tests {
    ($($t:ty)*) => ($(
        paste::paste! {
            #[test]
            fn [<test_to_string_ $t:snake>]() {
                #[allow(unused_comparisons)]
                fn to_string_naive(num: $t, radix: u8, reversed: bool, from_0: bool) -> String {
                    match (reversed, from_0) {
                        (false, false) => match radix {
                            10 => ToString::to_string(&num),
                            2 => if num < 0 {
                                if num == $t::MIN {
                                    let mut res = "-1".to_string();
                                    for _ in 0..$t::BITS - 1 {
                                        res.push('0');
                                    }
                                    res
                                } else {
                                    format!("-{:b}", $t::default() - num)
                                }
                            } else {
                                format!("{num:b}")
                            },
                            16 => if num < 0 {
                                if num == $t::MIN {
                                    let mut res = "-8".to_string();
                                    for _ in 0..($t::BITS / 4) - 1 {
                                        res.push('0');
                                    }
                                    res
                                } else {
                                    format!("-{:x}", $t::default() - num)
                                }
                            } else {
                                format!("{num:x}")
                            },
                            _ => unimplemented!(),
                        },
                        (true, false) => {
                            let s = to_string_naive(num, radix, false, false);
                            s.chars().rev().collect()
                        }
                        (reversed, true) => {
                            let s = to_string_naive(num, radix, reversed, false);
                            String::from_utf8(
                                s.bytes()
                                    .map(|b| match b {
                                        b'0'..=b'9' => b - b'0',
                                        b'-' => b'-',
                                        _ => unimplemented!(),
                                    })
                                    .collect(),
                            )
                            .unwrap()
                        }
                    }
                }

                fn test_num(n: $t) {
                    for radix in [10, 2, 16] {
                        for reversed in [false, true] {
                            for from_0 in [false, true] {
                                if from_0 && radix == 16 {
                                    continue;
                                }
                                let s = match radix {
                                    10 => n.to_string(reversed, from_0),
                                    2 => n.to_string_base2(reversed, from_0),
                                    16 => n.to_string_base16(reversed, from_0),
                                    _ => unreachable!(),
                                };
                                let s_naive = to_string_naive(n, radix, reversed, from_0);
                                assert_eq!(
                                    s, s_naive,
                                    "n = {n}, radix = {radix}, reversed = {reversed},\
                                    from_0 = {from_0}"
                                );
                                #[cfg(feature = "heapless")]
                                {
                                    let s_heapless = match radix {
                                        10 => n.to_heapless_string(reversed, from_0).to_string(),
                                        2 => {
                                            n.to_heapless_string_base2(reversed, from_0).to_string()
                                        }
                                        16 => n
                                            .to_heapless_string_base16(reversed, from_0)
                                            .to_string(),
                                        _ => unreachable!(),
                                    };
                                    assert_eq!(
                                        s_heapless, s_naive,
                                        "heapless n = {n}, radix = {radix}, reversed = {reversed},\
                                        from_0 = {from_0}"
                                    );
                                }
                            }
                        }
                    }
                }

                let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);

                for bits_shr in 0..$t::BITS {
                    for _ in 0..if cfg!(miri) { 1 } else { 100 } {
                        let n = rng.random::<$t>() >> bits_shr;
                        test_num(n);
                    }
                }

                test_num($t::MIN);
                test_num($t::MAX);
            }
        }
    )*)
}

gen_tests!(u128 i128 u64 i64 u32 i32 u16 i16 u8 i8);
