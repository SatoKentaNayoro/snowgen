use criterion::{black_box, criterion_group, criterion_main, Criterion};
use snowgen::{Epoch, SnowgenBuilder};
use std::sync::{Arc, Mutex};
use std::thread;

fn single_thread_benchmark(c: &mut Criterion) {
    let mut snowgen = SnowgenBuilder::new()
        .epoch(Epoch::MillisecondsSinceUnixEpoch(1632628649787))
        .node_id(1)
        .build()
        .unwrap();

    c.bench_function("single_thread_snowgen", |b| {
        b.iter(|| {
            black_box(snowgen.next_id().unwrap());
        })
    });
}

fn multi_thread_benchmark(c: &mut Criterion) {
    let snowgen = SnowgenBuilder::new()
        .epoch(Epoch::MillisecondsSinceUnixEpoch(1632628649787))
        .node_id(1)
        .build()
        .unwrap();

    let snowflake = Arc::new(Mutex::new(snowgen));

    c.bench_function("multi_thread_snowgen", |b| {
        b.iter(|| {
            let handles = (0..4)
                .map(|_| {
                    let snowgen = Arc::clone(&snowflake);
                    thread::spawn(move || {
                        let mut snowgen = snowgen.lock().unwrap();
                        black_box(snowgen.next_id().unwrap());
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
