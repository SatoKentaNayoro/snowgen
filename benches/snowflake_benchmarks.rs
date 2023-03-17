use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::{Arc, Mutex};
use std::thread;
use snowflake_rs::SnowflakeBuilder;
use snowflake_rs::Epoch;

fn single_thread_benchmark(c: &mut Criterion) {
    let mut snowflake = SnowflakeBuilder::new()
        .epoch(Epoch::MillisecondsSinceUnixEpoch(1632628649787))
        .node_id(1)
        .build()
        .unwrap();

    c.bench_function("single_thread_snowflake", |b| {
        b.iter(|| {
            black_box(snowflake.next_id().unwrap());
        })
    });
}

fn multi_thread_benchmark(c: &mut Criterion) {
    let snowflake = SnowflakeBuilder::new()
        .epoch(Epoch::MillisecondsSinceUnixEpoch(1632628649787))
        .node_id(1)
        .build()
        .unwrap();

    let snowflake = Arc::new(Mutex::new(snowflake));

    c.bench_function("multi_thread_snowflake", |b| {
        b.iter(|| {
            let handles = (0..4)
                .map(|_| {
                    let snowflake = Arc::clone(&snowflake);
                    thread::spawn(move || {
                        let mut snowflake = snowflake.lock().unwrap();
                        black_box(snowflake.next_id().unwrap());
                    })
                })
                .collect::<Vec<_>>();

            for handle in handles {
                handle.join().unwrap();
            }
        })
    });
}

criterion_group!(benches, single_thread_benchmark, multi_thread_benchmark);
criterion_main!(benches);
