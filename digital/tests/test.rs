use digital::prelude::*;
use rand::RngExt;
use rand_pcg::Pcg64Mcg;

macro_rules! gen_tests {
    ($($t:ty)*) => ($(
        paste::paste! {
            #[test]
            fn [<test_to_string_ $t>]() {
                #[allow(unused_comparisons)]
                fn to_string_naive<F: DigitFormat, R: Radix>(num: $t) -> String {
                    match (F::REVERSED, F::RAW) {
                        (false, false) => match R::BASE {
                            Base::Base10 => ToString::to_string(&num),
                            Base::Base2 => if num < 0 {
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
                            Base::Base16 => if num < 0 {
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
                        },
                        (true, false) => {
                            let s = to_string_naive::<Normal, R>(num);
                            s.chars().rev().collect()
                        }
                        (reversed, true) => {
                            let s = if reversed {
                                to_string_naive::<Reversed, R>(num)
                            } else {
                                to_string_naive::<Normal, R>(num)
                            };
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

                fn test_num_with<F: DigitFormat, R: Radix>(n: $t) {
                    let f = const {
                        match R::BASE {
                            Base::Base10 => <$t as IntToString<_, _, _>>::to_string_with::<F>,
                            Base::Base2 => <$t as IntToString<_, _, _>>::to_string_base2_with::<F>,
                            Base::Base16 => {
                                <$t as IntToString<_, _, _>>::to_string_base16_with::<F>
                            }
                        }
                    };
                    let s = f(n);
                    let s_naive = to_string_naive::<F, R>(n);
                    assert_eq!(
                        s, s_naive,
                        "n = {n}, radix = {:?}, reversed = {}, raw = {}",
                        R::BASE, F::REVERSED, F::RAW
                    );
                    #[cfg(feature = "heapless")]
                    {
                        let f = const {
                            match R::BASE {
                                Base::Base10 => {
                                    |n: $t| n.to_heapless_string_with::<F>().to_string()
                                }
                                Base::Base2 => {
                                    |n: $t| n.to_heapless_string_base2_with::<F>().to_string()
                                }
                                Base::Base16 => {
                                    |n: $t| n.to_heapless_string_base16_with::<F>().to_string()
                                }
                            }
                        };
                        let s_heapless = f(n);
                        assert_eq!(
                            s_heapless, s_naive,
                            "heapless n = {n}, radix = {:?}, reversed = {}, raw = {}",
                            R::BASE, F::REVERSED, F::RAW
                        );
                    }
                }

                fn test_num(n: $t) {
                    test_num_with::<Normal, Base10>(n);
                    test_num_with::<Reversed, Base10>(n);
                    test_num_with::<Raw, Base10>(n);
                    test_num_with::<ReversedRaw, Base10>(n);

                    test_num_with::<Normal, Base2>(n);
                    test_num_with::<Reversed, Base2>(n);
                    test_num_with::<Raw, Base2>(n);
                    test_num_with::<ReversedRaw, Base2>(n);

                    test_num_with::<Normal, Base16>(n);
                    test_num_with::<Reversed, Base16>(n);
                }

                let mut rng = Pcg64Mcg::new(0xcafe_f00d_d15e_a5e5);

                for bits_shr in 0..$t::BITS {
                    for _ in 0..if cfg!(miri) { 1 } else { 100 } {
                        let n = (rng.random::<u128>() as $t) >> bits_shr;
                        test_num(n);
                    }
                }

                test_num($t::MIN);
                test_num($t::MAX);
            }
        }
    )*)
}

gen_tests!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
);
