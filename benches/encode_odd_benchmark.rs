use azamcodec::azam_encode;
use azamcodec::encode::AzamEncode;
use criterion::{criterion_group, criterion_main, Criterion};
use uuid::Uuid;

fn test_decoder(instance: &mut Criterion) {
    instance.bench_function("uuid_to_string", |bencher| {
        // Data preparation
        let uuid_value = Uuid::parse_str("0fffffffffffffffffffffffffffffff").unwrap();

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = uuid_value.to_string();
        })
    });

    instance.bench_function("azam_encode_odd_u8", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xfu8.azam_encode();
        })
    });

    instance.bench_function("azam_encode_odd_u16", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xfffu16.azam_encode();
        })
    });

    instance.bench_function("azam_encode_odd_u32", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xfffffffu32.azam_encode();
        })
    });

    instance.bench_function("azam_encode_odd_u64", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xfffffffffffffffu64.azam_encode();
        })
    });

    instance.bench_function("azam_encode_odd_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = 0xfffffffffffffffffffffffffffffffu128.azam_encode();
        })
    });

    instance.bench_function("azam_encode_odd_macro_u8", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xfu8);
        })
    });

    instance.bench_function("azam_encode_odd_macro_u16", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xfffu16);
        })
    });

    instance.bench_function("azam_encode_odd_macro_u32", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xfffffffu32);
        })
    });

    instance.bench_function("azam_encode_odd_macro_u64", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xfffffffffffffffu64);
        })
    });

    instance.bench_function("azam_encode_odd_macro_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(0xfffffffffffffffffffffffffffffffu128);
        })
    });

    instance.bench_function("azam_encode_odd_macro_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(
                0xfffffffffffffffffffffffffffffffu128,
                0xfffffffffffffffffffffffffffffffu128
            );
        })
    });

    instance.bench_function("azam_encode_odd_macro_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(
                0xfffffffffffffffffffffffffffffffu128,
                0xfffffffffffffffffffffffffffffffu128,
                0xfffffffffffffffffffffffffffffffu128
            );
        })
    });

    instance.bench_function("azam_encode_odd_macro_u128_u128_u128_u128", |bencher| {
        // Data preparation

        // Benchmark iteration in closure
        bencher.iter(|| {
            let _x = azam_encode!(
                0xfffffffffffffffffffffffffffffffu128,
                0xfffffffffffffffffffffffffffffffu128,
                0xfffffffffffffffffffffffffffffffu128,
                0xfffffffffffffffffffffffffffffffu128
            );
        })
    });

    instance.bench_function(
        "azam_encode_odd_macro_u128_u128_u128_u128_u128",
        |bencher| {
            // Data preparation

            // Benchmark iteration in closure
            bencher.iter(|| {
                let _x = azam_encode!(
                    0xfffffffffffffffffffffffffffffffu128,
                    0xfffffffffffffffffffffffffffffffu128,
                    0xfffffffffffffffffffffffffffffffu128,
                    0xfffffffffffffffffffffffffffffffu128,
                    0xfffffffffffffffffffffffffffffffu128
                );
            })
        },
    );
}

criterion_group!(benches, test_decoder);
criterion_main!(benches);
