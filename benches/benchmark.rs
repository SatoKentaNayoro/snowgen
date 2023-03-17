use criterion::{black_box, criterion_group, criterion_main, Criterion};
use snowflake_rs::{Snowflake, SnowflakeBuilder};

fn bench_next_id(snowflake: &Snowflake) {
    for _ in 0..1000 {
        black_box(snowflake.next_id().unwrap());
    }
}

fn criterion_benchmark(c: &mut Criterion) {
    let snowflake = SnowflakeBuilder::new()
        .node_id(1)
        .timestamp_bits(41)
        .node_id_bits(5)
        .machine_id_bits(5)
        .sequence_bits(12)
        .build()
        .unwrap();

    c.bench_function("next_id", |b| b.iter(|| bench_next_id(&snowflake)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
