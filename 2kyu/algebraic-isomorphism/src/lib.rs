//! <https://www.codewars.com/kata/5917f22dd2563a36a200009c/train/rust>

#![no_std]

extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec};
use core::mem::{transmute, MaybeUninit};

#[derive(PartialEq, Eq)]
pub enum Void {}

pub type Func<A, B> = Box<dyn Fn(A) -> B>;
pub type RetFunc<A, B> = Box<dyn FnOnce(A) -> B>;
pub type ISO<A, B> = (Func<A, B>, Func<B, A>);

fn iso<A: 'static, B: 'static, F1, F2>(f1: F1, f2: F2) -> ISO<A, B>
where
    F1: 'static + Fn(A) -> B,
    F2: 'static + Fn(B) -> A,
{
    (Box::new(f1), Box::new(f2))
}

pub fn sub_st_l<A, B>(iso: ISO<A, B>) -> Func<A, B> {
    iso.0
}

pub fn sub_st_r<A, B>(iso: ISO<A, B>) -> Func<B, A> {
    iso.1
}

pub fn iso_bool() -> ISO<bool, bool> {
    iso(|a| a, |a| a)
}

pub fn iso_bool_not() -> ISO<bool, bool> {
    iso(|a: bool| !a, |a| !a)
}

pub fn refl<A: 'static>() -> ISO<A, A> {
    iso(|a| a, |a| a)
}

pub fn symm<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<B, A> {
    (i.1, i.0)
}

pub fn trans<A: 'static, B: 'static, C: 'static>(ab: ISO<A, B>, bc: ISO<B, C>) -> ISO<A, C> {
    let (a_to_b, b_to_a) = ab;
    let (b_to_c, c_to_b) = bc;
    iso(move |a| b_to_c(a_to_b(a)), move |c| b_to_a(c_to_b(c)))
}

pub fn iso_tuple<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    let (a_to_b, b_to_a) = ab;
    let (c_to_d, d_to_c) = cd;
    iso(
        move |(a, c)| (a_to_b(a), c_to_d(c)),
        move |(b, d)| (b_to_a(b), d_to_c(d)),
    )
}

pub fn iso_vec<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Vec<A>, Vec<B>> {
    let (a_to_b, b_to_a) = i;
    iso(
        move |v: Vec<_>| {
            let mut res = Vec::with_capacity(v.len());
            unsafe { res.set_len(v.len()) };
            let mut res_ptr = res.as_mut_ptr();
            for a in v {
                unsafe {
                    *res_ptr = MaybeUninit::new(a_to_b(a));
                    res_ptr = res_ptr.add(1);
                }
            }
            unsafe { transmute(res) }
        },
        move |v: Vec<_>| {
            let mut res = Vec::with_capacity(v.len());
            unsafe { res.set_len(v.len()) };
            let mut res_ptr = res.as_mut_ptr();
            for b in v {
                unsafe {
                    *res_ptr = MaybeUninit::new(b_to_a(b));
                    res_ptr = res_ptr.add(1);
                }
            }
            unsafe { transmute(res) }
        },
    )
}

pub fn iso_option<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    let (a_to_b, b_to_a) = i;
    iso(
        move |maybe_a: Option<_>| maybe_a.map(&a_to_b),
        move |maybe_b: Option<_>| maybe_b.map(&b_to_a),
    )
}

pub fn iso_result<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    let (a_to_b, b_to_a) = ab;
    let (c_to_d, d_to_c) = cd;
    iso(
        move |a_or_c: Result<_, _>| a_or_c.map(&a_to_b).map_err(&c_to_d),
        move |b_or_d: Result<_, _>| b_or_d.map(&b_to_a).map_err(&d_to_c),
    )
}

pub fn iso_un_option<A: 'static, B: 'static>(i: ISO<Option<A>, Option<B>>) -> ISO<A, B> {
    let (maybe_a_to_maybe_b, maybe_b_to_maybe_a) = i;
    iso(
        move |a| maybe_a_to_maybe_b(Some(a)).unwrap_or_else(|| maybe_a_to_maybe_b(None).unwrap()),
        move |b| maybe_b_to_maybe_a(Some(b)).unwrap_or_else(|| maybe_b_to_maybe_a(None).unwrap()),
    )
}

pub fn iso_eu() -> ISO<Result<Vec<()>, ()>, Result<Vec<()>, Void>> {
    iso(
        |v: Result<Vec<_>, _>| match v {
            Ok(mut v) => {
                v.push(());
                Ok(v)
            }
            Err(()) => Ok(vec![]),
        },
        |v| match v {
            Ok(mut v) if !v.is_empty() => {
                v.pop();
                Ok(v)
            }
            Ok(_) => Err(()),
            Err(_) => unreachable!(),
        },
    )
}

pub fn iso_symm<A: 'static, B: 'static>() -> ISO<ISO<A, B>, ISO<B, A>> {
    iso(symm, symm)
}

pub fn iso_prod<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<(A, C), (B, D)> {
    iso_tuple(ab, cd)
}

pub fn iso_plus<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> ISO<Result<A, C>, Result<B, D>> {
    iso_result(ab, cd)
}

pub fn iso_s<A: 'static, B: 'static>(i: ISO<A, B>) -> ISO<Option<A>, Option<B>> {
    iso_option(i)
}

pub type IsoFL<A, B, C, D> = RetFunc<Func<A, C>, RetFunc<B, D>>;
pub type IsoFR<A, B, C, D> = RetFunc<Func<B, D>, RetFunc<A, C>>;
pub type IsoF<A, B, C, D> = (IsoFL<A, B, C, D>, IsoFR<A, B, C, D>);

fn iso_f<A: 'static, B: 'static, C: 'static, D: 'static, F1, F2>(f1: F1, f2: F2) -> IsoF<A, B, C, D>
where
    F1: 'static + FnOnce(Func<A, C>) -> RetFunc<B, D>,
    F2: 'static + FnOnce(Func<B, D>) -> RetFunc<A, C>,
{
    (Box::new(f1), Box::new(f2))
}

pub fn iso_func<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> IsoF<A, B, C, D> {
    let (a_to_b, b_to_a) = ab;
    let (c_to_d, d_to_c) = cd;
    iso_f(
        move |a_to_c| Box::new(move |b| c_to_d(a_to_c(b_to_a(b)))),
        move |b_to_d| Box::new(move |a| d_to_c(b_to_d(a_to_b(a)))),
    )
}

pub fn iso_pow<A: 'static, B: 'static, C: 'static, D: 'static>(
    ab: ISO<A, B>,
    cd: ISO<C, D>,
) -> IsoF<A, B, C, D> {
    iso_func(ab, cd)
}

pub fn plus_comm<A: 'static, B: 'static>() -> ISO<Result<A, B>, Result<B, A>> {
    iso(
        |a_or_b| match a_or_b {
            Ok(a) => Err(a),
            Err(b) => Ok(b),
        },
        |b_or_a| match b_or_a {
            Ok(b) => Err(b),
            Err(a) => Ok(a),
        },
    )
}

pub fn plus_assoc<A: 'static, B: 'static, C: 'static>(
) -> ISO<Result<Result<A, B>, C>, Result<A, Result<B, C>>> {
    iso(
        |a_or_b_or_c| match a_or_b_or_c {
            Ok(Ok(a)) => Ok(a),
            Ok(Err(b)) => Err(Ok(b)),
            Err(c) => Err(Err(c)),
        },
        |a_or_b_or_c| match a_or_b_or_c {
            Ok(a) => Ok(Ok(a)),
            Err(Ok(b)) => Ok(Err(b)),
            Err(Err(c)) => Err(c),
        },
    )
}

pub fn mult_comm<A: 'static, B: 'static>() -> ISO<(A, B), (B, A)> {
    iso(|(a, b)| (b, a), |(b, a)| (a, b))
}

pub fn mult_assoc<A: 'static, B: 'static, C: 'static>() -> ISO<((A, B), C), (A, (B, C))> {
    iso(|((a, b), c)| (a, (b, c)), |(a, (b, c))| ((a, b), c))
}

pub fn dist<A: 'static, B: 'static, C: 'static>() -> ISO<(A, Result<B, C>), Result<(A, B), (A, C)>>
{
    iso(
        move |(a, b_or_c)| match b_or_c {
            Ok(b) => Ok((a, b)),
            Err(c) => Err((a, c)),
        },
        move |ab_or_ac| match ab_or_ac {
            Ok((a, b)) => (a, Ok(b)),
            Err((a, c)) => (a, Err(c)),
        },
    )
}

pub type IsoCL<A, B, C> = RetFunc<Func<A, Func<B, C>>, RetFunc<(A, B), C>>;
pub type IsoCR<A, B, C> = RetFunc<Func<(A, B), C>, RetFunc<A, RetFunc<B, C>>>;
pub type IsoC<A, B, C> = (IsoCL<A, B, C>, IsoCR<A, B, C>);

fn iso_c<A: 'static, B: 'static, C: 'static, F1, F2>(f1: F1, f2: F2) -> IsoC<A, B, C>
where
    F1: 'static + FnOnce(Func<A, Func<B, C>>) -> RetFunc<(A, B), C>,
    F2: 'static + FnOnce(Func<(A, B), C>) -> RetFunc<A, RetFunc<B, C>>,
{
    (Box::new(f1), Box::new(f2))
}

pub fn curry_iso<A: 'static, B: 'static, C: 'static>() -> IsoC<A, B, C> {
    iso_c(
        move |a_to_b_to_c| Box::new(move |(a, b)| a_to_b_to_c(a)(b)),
        move |ab_to_c| Box::new(move |a| Box::new(move |b| ab_to_c((a, b)))),
    )
}

pub fn one() -> ISO<(), Option<Void>> {
    iso(|()| None, |_| ())
}

pub fn two() -> ISO<bool, Option<Option<Void>>> {
    iso(|b| if b { Some(None) } else { None }, |oov| oov.is_some())
}

pub fn plus_o<B: 'static>() -> ISO<Result<Void, B>, B> {
    iso(
        |or_b: Result<_, _>| unsafe { or_b.unwrap_err_unchecked() },
        |b| Err(b),
    )
}

pub fn plus_s<A: 'static, B: 'static>() -> ISO<Result<Option<A>, B>, Option<Result<A, B>>> {
    iso(
        |maybe_a_or_b| match maybe_a_or_b {
            Ok(Some(a)) => Some(Ok(a)),
            Err(b) => Some(Err(b)),
            Ok(None) => None,
        },
        |maybe_a_or_b| match maybe_a_or_b {
            Some(Ok(a)) => Ok(Some(a)),
            Some(Err(b)) => Err(b),
            None => Ok(None),
        },
    )
}

pub fn plus_so<B: 'static>() -> ISO<Result<(), B>, Option<B>> {
    iso(
        |or_b| if let Err(b) = or_b { Some(b) } else { None },
        |maybe_b| maybe_b.map_or(Ok(()), |b| Err(b)),
    )
}

pub fn mult_o<A: 'static>() -> ISO<(Void, A), Void> {
    iso(|(_, _)| unreachable!(), |_| unreachable!())
}

pub fn mult_s<A: 'static, B: 'static>() -> ISO<(Option<A>, B), Result<B, (A, B)>> {
    iso(
        |(maybe_a, b)| {
            if let Some(a) = maybe_a {
                Err((a, b))
            } else {
                Ok(b)
            }
        },
        |b_or_ab| match b_or_ab {
            Ok(b) => (None, b),
            Err((a, b)) => (Some(a), b),
        },
    )
}

pub fn mult_so<B: 'static>() -> ISO<((), B), B> {
    iso(|(_, b)| b, |b| ((), b))
}

pub type IsoPL<A> = RetFunc<Func<Void, A>, ()>;
pub type IsoPR<A> = RetFunc<(), RetFunc<Void, A>>;
pub type IsoP<A> = (IsoPL<A>, IsoPR<A>);

fn iso_p<A: 'static, F1, F2>(f1: F1, f2: F2) -> IsoP<A>
where
    F1: 'static + FnOnce(Func<Void, A>),
    F2: 'static + FnOnce(()) -> RetFunc<Void, A>,
{
    (Box::new(f1), Box::new(f2))
}

pub fn pow_o<A: 'static>() -> IsoP<A> {
    iso_p(|_| (), |_| Box::new(|_| unreachable!()))
}

pub type IsoPsL<A, B> = RetFunc<Func<Option<B>, A>, (A, RetFunc<B, A>)>;
pub type IsoPsR<A, B> = RetFunc<(A, Func<B, A>), RetFunc<Option<B>, A>>;
pub type IsoPs<A, B> = (IsoPsL<A, B>, IsoPsR<A, B>);

fn iso_ps<A: 'static, B: 'static, F1, F2>(f1: F1, f2: F2) -> IsoPs<A, B>
where
    F1: 'static + FnOnce(Func<Option<B>, A>) -> (A, RetFunc<B, A>),
    F2: 'static + FnOnce((A, Func<B, A>)) -> RetFunc<Option<B>, A>,
{
    (Box::new(f1), Box::new(f2))
}

pub fn pow_s<A: 'static, B: 'static>() -> IsoPs<A, B> {
    iso_ps(
        move |maybe_b_to_a| (maybe_b_to_a(None), Box::new(move |b| maybe_b_to_a(Some(b)))),
        move |(a, b_to_a)| Box::new(move |maybe_b| maybe_b.map_or(a, b_to_a)),
    )
}

pub type IsoPsoL<A> = RetFunc<Func<(), A>, A>;
pub type IsoPsoR<A> = RetFunc<A, RetFunc<(), A>>;
pub type IsoPso<A> = (IsoPsoL<A>, IsoPsoR<A>);

fn iso_pso<A: 'static, F1, F2>(f1: F1, f2: F2) -> IsoPso<A>
where
    F1: 'static + FnOnce(Func<(), A>) -> A,
    F2: 'static + FnOnce(A) -> RetFunc<(), A>,
{
    (Box::new(f1), Box::new(f2))
}

pub fn pow_so<A: 'static>() -> IsoPso<A> {
    iso_pso(|to_a| to_a(()), |a| Box::new(|()| a))
}
