use std::time::{Duration, Instant};
use snowflake_rs::{Epoch, SnowflakeBuilder};

#[test]
fn test_snowflake_generation() {
    let snowflake = SnowflakeBuilder::new()
        .node_id(1)
        .epoch(Epoch::SystemTime(std::time::UNIX_EPOCH))
        .build()
        .unwrap();

    let id1 = snowflake.next_id().unwrap();
    let id2 = snowflake.next_id().unwrap();
    assert_ne!(id1, id2);
}

#[test]
fn test_snowflake_id_uniqueness() {
    let snowflake = SnowflakeBuilder::new()
        .node_id(1)
        .epoch(Epoch::SystemTime(std::time::UNIX_EPOCH))
        .build()
        .unwrap();

    let mut ids = std::collections::HashSet::new();
    for _ in 0..1000 {
        ids.insert(snowflake.next_id().unwrap());
    }

    assert_eq!(ids.len(), 1000);
}

#[test]
fn test_snowflake_custom_parameters() {
    let snowflake = SnowflakeBuilder::new()
        .node_id(3)
        .machine_id(2)
        .epoch(Epoch::Instant(Instant::now() - Duration::from_secs(60)))
        .timestamp_bits(42)
        .node_id_bits(6)
        .machine_id_bits(6)
        .sequence_bits(10)
        .build()
        .unwrap();

    let id1 = snowflake.next_id().unwrap();
    let id2 = snowflake.next_id().unwrap();
    assert_ne!(id1, id2);
}

#[test]
fn test_snowflake_invalid_parameters() {
    let builder = SnowflakeBuilder::new()
        .node_id(32)
        .machine_id(32)
        .timestamp_bits(42)
        .node_id_bits(12)
        .machine_id_bits(8)
        .sequence_bits(10);

    assert!(builder.build().is_err());
}

#[test]
fn test_snowflake_invalid_node_id() {
    let builder = SnowflakeBuilder::new()
        .node_id(-1)
        .machine_id(0)
        .timestamp_bits(41)
        .node_id_bits(5)
        .machine_id_bits(5)
        .sequence_bits(12);

    assert!(builder.build().is_err());
}

#[test]
fn test_snowflake_invalid_machine_id() {
    let builder = SnowflakeBuilder::new()
        .node_id(0)
        .machine_id(-1)
        .timestamp_bits(41)
        .node_id_bits(5)
        .machine_id_bits(5)
        .sequence_bits(12);

    assert!(builder.build().is_err());
}

#[test]
fn test_snowflake_exceeding_bit_sum() {
    let builder = SnowflakeBuilder::new()
        .node_id(0)
        .machine_id(0)
        .timestamp_bits(42)
        .node_id_bits(12)
        .machine_id_bits(8)
        .sequence_bits(10);

    assert!(builder.build().is_err());
}
