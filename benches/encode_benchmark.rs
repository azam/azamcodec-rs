use azamcodec::azam_encode;
use azamcodec::encode::AzamEncode;
use criterion::{criterion_group, criterion_main, Criterion};
use uuid::Uuid;

fn test_decoder(instance: &mut Criterion) {
    instance.bench_function("uuid_to_string", |bencher| {
        // Data preparation
        let uuid_value = Uuid::parse_str("ffffffffffffffffffffffffffffffff").unwrap();

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = uuid_value.to_string();
        })
    });

    instance.bench_function("azam_encode_u8", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xffu8.azam_encode();
        })
    });

    instance.bench_function("azam_encode_u16", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xffffu16.azam_encode();
        })
    });

    instance.bench_function("azam_encode_u32", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xffffffffu32.azam_encode();
        })
    });

    instance.bench_function("azam_encode_u64", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xffffffffffffffffu64.azam_encode();
        })
    });

    instance.bench_function("azam_encode_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xffffffffffffffffffffffffffffffffu128.azam_encode();
        })
    });

    instance.bench_function("azam_encode_macro_u8", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xffu8);
        })
    });

    instance.bench_function("azam_encode_macro_u16", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xffffu16);
        })
    });

    instance.bench_function("azam_encode_macro_u32", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xffffffffu32);
        })
    });

    instance.bench_function("azam_encode_macro_u64", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xffffffffffffffffu64);
        })
    });

    instance.bench_function("azam_encode_macro_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xffffffffffffffffffffffffffffffffu128);
        })
    });

    instance.bench_function("azam_encode_macro_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128
            );
        })
    });

    instance.bench_function("azam_encode_macro_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128
            );
        })
    });

    instance.bench_function("azam_encode_macro_u128_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128
            );
        })
    });

    instance.bench_function("azam_encode_macro_u128_u128_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128,
                0xffffffffffffffffffffffffffffffffu128
            );
        })
    });
}

criterion_group!(benches, test_decoder);
criterion_main!(benches);
