use core::{iter::Sum, time::Duration};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use digital::{MaxLenBase10, NumToString};
use num_traits::PrimInt;

fn bench_to_heapless_string(c: &mut Criterion) {
    fn bench_type<
        T: MaxLenBase10 + NumToString<CAP10, CAP2, CAP16> + From<u8> + PrimInt + Sum,
        const CAP10: usize,
        const CAP2: usize,
        const CAP16: usize,
    >(
        c: &mut Criterion,
        type_str: &str,
    ) {
        let mut group = c.benchmark_group(type_str);
        group
            .warm_up_time(Duration::from_millis(500))
            .measurement_time(Duration::from_secs(1));
        for size in 1..=T::MAX_LEN_BASE10 as u32 {
            let n: T = (0..size)
                .map(|pow| <T as From<_>>::from(10u8).pow(pow))
                .sum();
            group.bench_with_input(BenchmarkId::from_parameter(size), &n, |b, &n| {
                b.iter(|| n.to_heapless_string(false, false));
            });
        }
        group.finish();
    }

    bench_type::<u8, { u8::MAX_LEN_BASE10 }, { u8::BITS as _ }, { (u8::BITS / 4) as _ }>(c, "u8");
    bench_type::<u16, { u16::MAX_LEN_BASE10 }, { u16::BITS as _ }, { (u16::BITS / 4) as _ }>(
        c, "u16",
    );
    bench_type::<u32, { u32::MAX_LEN_BASE10 }, { u32::BITS as _ }, { (u32::BITS / 4) as _ }>(
        c, "u32",
    );
    bench_type::<u64, { u64::MAX_LEN_BASE10 }, { u64::BITS as _ }, { (u64::BITS / 4) as _ }>(
        c, "u64",
    );
    bench_type::<u128, { u128::MAX_LEN_BASE10 }, { u128::BITS as _ }, { (u128::BITS / 4) as _ }>(
        c, "u128",
    );
}

criterion_group!(benches, bench_to_heapless_string);
criterion_main!(benches);
