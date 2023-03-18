use std::sync::{Arc, Mutex};
use std::thread;
use snowflake_rs::SnowflakeBuilder;

#[test]
fn test_snowflake_unique_ids() {
    let snowflake = SnowflakeBuilder::new()
        .node_id(1)
        .build()
        .unwrap();
    let snowflake = Arc::new(Mutex::new(snowflake));
    let mut ids = std::collections::HashSet::new();

    for _ in 0..10000 {
        let mut snowflake = snowflake.lock().unwrap();
        let id = snowflake.next_id().unwrap();
        drop(snowflake);
        assert!(ids.insert(id), "ID duplicate：{}", id);
    }
}

#[test]
fn test_snowflake_unique_ids_multithread() {
    let snowflake = SnowflakeBuilder::new()
        .node_id(1)
        .build()
        .unwrap();
    let snowflake = Arc::new(Mutex::new(snowflake));
    let mut handles = vec![];

    for _ in 0..10 {
        let snowflake = Arc::clone(&snowflake);
        let handle = thread::spawn(move || {
            let mut ids = std::collections::HashSet::new();
            for _ in 0..1000 {
                let mut snowflake = snowflake.lock().unwrap();
                let id = snowflake.next_id().unwrap();
                drop(snowflake);
                assert!(ids.insert(id), "ID duplicate：{}", id);
            }
            ids
        });
        handles.push(handle);
    }

    let mut all_ids = std::collections::HashSet::new();
    for handle in handles {
        let ids = handle.join().unwrap();
        for id in ids {
            assert!(all_ids.insert(id), "ID duplicate：{}", id);
        }
    }
}