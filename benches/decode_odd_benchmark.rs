use azamcodec::azam_decode;
use azamcodec::decode::AzamDecode;
use criterion::{criterion_group, criterion_main, Criterion};
use uuid::Uuid;

fn test_decoder(instance: &mut Criterion) {
    instance.bench_function("uuid_parse_str_odd", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = Uuid::parse_str("0fffffffffffffffffffffffffffffff").unwrap();
        })
    });

    instance.bench_function("azam_decode_odd_u8", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u8::azam_decode("f");
        })
    });

    instance.bench_function("azam_decode_odd_u16", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u16::azam_decode("zzf");
        })
    });

    instance.bench_function("azam_decode_odd_u32", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u32::azam_decode("zzzzzzf");
        })
    });

    instance.bench_function("azam_decode_odd_u64", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u64::azam_decode("zzzzzzzzzzzzzzf");
        })
    });

    instance.bench_function("azam_decode_odd_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u128::azam_decode("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf");
        })
    });

    instance.bench_function("azam_decode_odd_macro_u8", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("f", u8);
        })
    });

    instance.bench_function("azam_decode_odd_macro_u16", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzf", u16);
        })
    });

    instance.bench_function("azam_decode_odd_macro_u32", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzf", u32);
        })
    });

    instance.bench_function("azam_decode_odd_macro_u64", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzf", u64);
        })
    });

    instance.bench_function("azam_decode_odd_macro_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128);
        })
    });

    instance.bench_function("azam_decode_odd_macro_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128, u128);
        })
    });

    instance.bench_function("azam_decode_odd_macro_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128, u128, u128);
        })
    });

    instance.bench_function("azam_decode_odd_macro_u128_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128, u128, u128, u128);
        })
    });

    instance.bench_function("azam_decode_odd_macro_u128_u128_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128, u128, u128, u128, u128);
        })
    });
}

criterion_group!(benches, test_decoder);
criterion_main!(benches);
