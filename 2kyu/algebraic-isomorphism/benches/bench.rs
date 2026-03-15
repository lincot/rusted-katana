#![feature(test)]

extern crate test;
use algebraic_isomorphism::{
    dist, iso, iso_bool, iso_bool_not, iso_eu, iso_option, iso_result, iso_symm, iso_tuple,
    iso_un_option, iso_vec, mult_s, mult_so, one, plus_assoc, plus_o, plus_s, plus_so, refl,
    sub_st_l, sub_st_r, symm, trans, two, ISO,
};
use test::{black_box, Bencher};

fn moe() -> String {
    "MOE".to_string()
}

fn meow() -> String {
    "MEOW".to_string()
}

fn verbose() -> String {
    "It was me, DIO!".to_string()
}

fn lu() -> Vec<()> {
    vec![(), (), (), (), (), (), (), (), ()]
}

fn t_iso() -> ISO<bool, String> {
    iso(
        move |x| if x { meow() } else { moe() },
        move |x| meow() == x,
    )
}

fn m_iso() -> ISO<Option<bool>, Option<bool>> {
    iso(
        move |it: Option<bool>| it.map_or(Some(true), |b| if b { Some(false) } else { None }),
        move |it: Option<bool>| it,
    )
}

fn b_iso() -> ISO<bool, bool> {
    iso_un_option(m_iso())
}

fn bad_m_iso() -> ISO<Option<bool>, Option<bool>> {
    iso(
        move |it: Option<bool>| it.and_then(|b| if b { Some(false) } else { None }),
        move |it: Option<bool>| it,
    )
}

fn bad_iso() -> ISO<bool, bool> {
    iso_un_option(bad_m_iso())
}

fn lrl<A: 'static, B: 'static>(i: ISO<A, B>, a: A) -> A {
    let (fw, bw) = i;
    bw(fw(a))
}

fn rlr<A: 'static, B: 'static>(i: ISO<A, B>, b: B) -> B {
    let (fw, bw) = i;
    fw(bw(b))
}

#[bench]
#[allow(unused)]
fn bench(bencher: &mut Bencher) {
    bencher.iter(|| {
        black_box(sub_st_l(black_box(iso_bool()))(black_box(true)));
        black_box(sub_st_l(black_box(iso_bool()))(black_box(false)));
        black_box(sub_st_l(black_box(iso_bool_not()))(black_box(false)));
        black_box(sub_st_l(black_box(t_iso()))(black_box(true)));

        black_box(sub_st_r(black_box(iso_bool()))(black_box(true)));
        black_box(sub_st_r(black_box(iso_bool()))(black_box(false)));
        black_box(sub_st_r(black_box(t_iso()))(black_box(meow())));

        black_box(sub_st_l(sub_st_l(black_box(refl()))(black_box(t_iso())))(
            black_box(true),
        ));

        black_box(sub_st_r(black_box(symm(black_box(t_iso()))))(black_box(
            true,
        )));

        black_box(sub_st_l(trans(
            black_box(refl()),
            trans(black_box(t_iso()), black_box(refl())),
        ))(black_box(true)));

        black_box(sub_st_l(iso_tuple(black_box(t_iso()), black_box(t_iso())))(
            (black_box(true), black_box(false)),
        ));

        black_box(sub_st_l(iso_vec(black_box(t_iso())))(black_box(vec![
            true, true, false,
        ])));

        black_box(sub_st_l(iso_option(black_box(t_iso())))(black_box(None)));
        black_box(sub_st_l(iso_option(black_box(t_iso())))(black_box(Some(
            true,
        ))));

        black_box(sub_st_l(black_box(b_iso()))(black_box(true)));
        black_box(sub_st_l(black_box(b_iso()))(black_box(false)));
        black_box(sub_st_l(black_box(bad_iso()))(black_box(true)));

        black_box(sub_st_l(iso_result(
            black_box(iso_bool_not()),
            black_box(t_iso()),
        ))(black_box(Ok(true))));
        black_box(sub_st_l(iso_result(
            black_box(iso_bool_not()),
            black_box(t_iso()),
        ))(black_box(Ok(false))));
        black_box(sub_st_l(iso_result(
            black_box(iso_bool_not()),
            black_box(t_iso()),
        ))(black_box(Err(true))));

        black_box(sub_st_r(sub_st_r(black_box(iso_symm()))(
            black_box(t_iso()),
        ))(black_box(false)));

        black_box(lrl(black_box(iso_eu()), black_box(Ok(vec![]))));
        black_box(lrl(black_box(iso_eu()), black_box(Ok(lu()))));
        black_box(lrl(black_box(iso_eu()), black_box(Err(()))));
        black_box(rlr(black_box(iso_eu()), black_box(Ok(vec![]))));
        black_box(rlr(black_box(iso_eu()), black_box(Ok(lu()))));
        black_box(sub_st_l(black_box(iso_eu()))(black_box(Err(()))));

        black_box(lrl(
            black_box(plus_assoc()),
            black_box(Ok::<Result<i16, bool>, String>(Ok(233))),
        ));
        black_box(lrl(
            black_box(plus_assoc()),
            black_box(Ok::<Result<i16, bool>, String>(Err(true))),
        ));
        black_box(lrl(
            black_box(plus_assoc()),
            black_box(Err::<Result<i16, bool>, String>(verbose())),
        ));
        black_box(rlr(
            black_box(plus_assoc()),
            black_box(Ok::<i16, Result<bool, String>>(233)),
        ));
        black_box(rlr(
            black_box(plus_assoc()),
            black_box(Err::<i16, Result<bool, String>>(Ok(true))),
        ));
        black_box(rlr(
            black_box(plus_assoc()),
            black_box(Err::<i16, Result<bool, String>>(Err(verbose()))),
        ));

        black_box(lrl(
            black_box(dist()),
            black_box((233, Ok::<bool, String>(true))),
        ));
        black_box(lrl(
            black_box(dist()),
            black_box((233, Err::<bool, String>(verbose()))),
        ));
        black_box(rlr(
            black_box(dist()),
            black_box(Ok::<(i16, bool), (i16, String)>((0, true))),
        ));
        black_box(rlr(
            black_box(dist()),
            black_box(Err::<(i16, bool), (i16, String)>((0, verbose()))),
        ));

        black_box(rlr(black_box(one()), black_box(None)));

        black_box(lrl(black_box(two()), black_box(true)));
        black_box(lrl(black_box(two()), black_box(false)));
        black_box(rlr(black_box(two()), black_box(None)));
        black_box(rlr(black_box(two()), black_box(Some(None))));

        black_box(lrl(black_box(plus_o()), black_box(Err(verbose()))));
        black_box(rlr(black_box(plus_o()), black_box(verbose())));
        black_box(lrl(
            black_box(plus_s()),
            black_box(Ok::<Option<String>, i16>(Some(verbose()))),
        ));
        black_box(lrl(
            black_box(plus_s()),
            black_box(Ok::<Option<String>, i16>(None)),
        ));
        black_box(lrl(
            black_box(plus_s()),
            black_box(Err::<Option<String>, i16>(233)),
        ));
        black_box(rlr(
            black_box(plus_s()),
            black_box(Some::<Result<String, i16>>(Ok(verbose()))),
        ));
        black_box(rlr(
            black_box(plus_s()),
            black_box(Some::<Result<String, i16>>(Err(233))),
        ));
        black_box(rlr(
            black_box(plus_s()),
            black_box(None::<Result<String, i16>>),
        ));
        black_box(lrl(black_box(plus_so()), black_box(Ok::<(), i16>(()))));
        black_box(lrl(black_box(plus_so()), black_box(Err::<(), i16>(233))));
        black_box(rlr(black_box(plus_so()), black_box(None::<i16>)));
        black_box(rlr(black_box(plus_so()), black_box(Some(233))));

        black_box(lrl(black_box(mult_s()), black_box((Some(verbose()), 233))));
        black_box(lrl(black_box(mult_s()), black_box((None::<()>, 233))));
        black_box(rlr(
            black_box(mult_s()),
            black_box(Ok::<i16, (String, i16)>(233)),
        ));
        black_box(rlr(
            black_box(mult_s()),
            black_box(Err::<i16, (String, i16)>((verbose(), 233))),
        ));
        black_box(lrl(black_box(mult_so()), black_box(((), true))));
        black_box(lrl(black_box(mult_so()), black_box(((), false))));
        black_box(rlr(black_box(mult_so()), black_box(true)));
        black_box(rlr(black_box(mult_so()), black_box(false)));
    });
}
