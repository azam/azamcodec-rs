use azamcodec::azam_decode;
use azamcodec::decode::AzamDecode;
use criterion::{criterion_group, criterion_main, Criterion};

fn test_decoder(instance: &mut Criterion) {
    instance.bench_function("azam_decode_u8", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u8::azam_decode("zf");
        })
    });

    instance.bench_function("azam_decode_u16", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u16::azam_decode("zzzf");
        })
    });

    instance.bench_function("azam_decode_u32", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u32::azam_decode("zzzzzzzf");
        })
    });

    instance.bench_function("azam_decode_u64", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u64::azam_decode("zzzzzzzzzzzzzzzf");
        })
    });

    instance.bench_function("azam_decode_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = u128::azam_decode("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf");
        })
    });

    instance.bench_function("azam_decode_macro_u8", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zf", u8);
        })
    });

    instance.bench_function("azam_decode_macro_u16", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzf", u16);
        })
    });

    instance.bench_function("azam_decode_macro_u32", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzf", u32);
        })
    });

    instance.bench_function("azam_decode_macro_u64", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzf", u64);
        })
    });

    instance.bench_function("azam_decode_macro_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128);
        })
    });

    instance.bench_function("azam_decode_macro_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128, u128);
        })
    });

    instance.bench_function("azam_decode_macro_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128, u128, u128);
        })
    });

    instance.bench_function("azam_decode_macro_u128_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128, u128, u128, u128);
        })
    });

    instance.bench_function("azam_decode_macro_u128_u128_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_decode!("zzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzfzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzzf", u128, u128, u128, u128, u128);
        })
    });
}

criterion_group!(benches, test_decoder);
criterion_main!(benches);
