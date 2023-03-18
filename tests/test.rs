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


#[test]
fn test_build_valid_configuration() {
    let builder = SnowgenBuilder::new().node_id(1).build();
    assert!(builder.is_ok());
}

#[test]
fn test_build_exceed_max_bits() {
    let builder = SnowgenBuilder::new()
        .timestamp_bits(41)
        .node_id_bits(12)
        .machine_id_bits(12)
        .sequence_bits(1)
        .build();
    assert_eq!(
        builder.err().unwrap(),
        "The sum of timestamp_bits, node_id_bits, machine_id_bits, and sequence_bits should not exceed 64."
    );
}

#[test]
fn test_build_invalid_node_id() {
    let builder = SnowgenBuilder::new().node_id(-1).build();
    assert_eq!(
        builder.err().unwrap(),
        "Invalid node_id, it should be between 0 and the maximum node_id."
    );

    let builder = SnowgenBuilder::new().node_id(33).build();
    assert_eq!(
        builder.err().unwrap(),
        "Invalid node_id, it should be between 0 and the maximum node_id."
    );
}

#[test]
fn test_build_invalid_machine_id() {
    let builder = SnowgenBuilder::new().machine_id(Some(-1)).build();
    assert_eq!(
        builder.err().unwrap(),
        "Invalid machine_id, it should be between 0 and the maximum machine_id."
    );

    let builder = SnowgenBuilder::new()
        .machine_id(Some(33))
        .machine_id_bits(5)
        .build();
    assert_eq!(
        builder.err().unwrap(),
        "Invalid machine_id, it should be between 0 and the maximum machine_id."
    );
}