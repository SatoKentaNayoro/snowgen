use snowflake_rs::{Epoch, SnowflakeBuilder};
use std::sync::{Arc, Barrier};
use std::thread;

#[test]
fn test_multithread_safety() {
    let snowflake = SnowflakeBuilder::new()
        .node_id(1)
        .epoch(Epoch::SystemTime(std::time::UNIX_EPOCH))
        .build()
        .unwrap();
    let snowflake = Arc::new(snowflake);
    let num_threads = 10;
    let num_ids_per_thread = 1000;

    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = Vec::with_capacity(num_threads);

    for _ in 0..num_threads {
        let snowflake_clone = Arc::clone(&snowflake);
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            let mut ids = Vec::with_capacity(num_ids_per_thread);
            barrier_clone.wait(); // Synchronize the start of ID generation

            for _ in 0..num_ids_per_thread {
                let id = snowflake_clone.next_id().unwrap();
                ids.push(id);
            }
            ids
        });
        handles.push(handle);
    }

    let mut all_generated_ids = Vec::new();
    for handle in handles {
        all_generated_ids.extend(handle.join().unwrap());
    }

    // Ensure all generated IDs are unique
    all_generated_ids.sort_unstable();
    all_generated_ids.dedup();
    assert_eq!(all_generated_ids.len(), num_threads * num_ids_per_thread);
}
