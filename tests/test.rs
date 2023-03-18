use snowgen::SnowgenBuilder;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn test_snowgen_unique_ids() {
    let snowgen = SnowgenBuilder::new().node_id(1).build().unwrap();
    let snowgen = Arc::new(Mutex::new(snowgen));
    let mut ids = std::collections::HashSet::new();

    for _ in 0..10000 {
        let mut snowgen = snowgen.lock().unwrap();
        let id = snowgen.next_id().unwrap();
        drop(snowgen);
        assert!(ids.insert(id), "ID duplicate：{}", id);
    }
}

#[test]
fn test_snowflake_unique_ids_multithread() {
    let snowgen = SnowgenBuilder::new().node_id(1).build().unwrap();
    let snowgen = Arc::new(Mutex::new(snowgen));
    let mut handles = vec![];

    for _ in 0..10 {
        let snowgen = Arc::clone(&snowgen);
        let handle = thread::spawn(move || {
            let mut ids = std::collections::HashSet::new();
            for _ in 0..1000 {
                let mut snowgen = snowgen.lock().unwrap();
                let id = snowgen.next_id().unwrap();
                drop(snowgen);
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
